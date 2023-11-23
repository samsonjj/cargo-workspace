use anyhow::{Context, Result};
use std::collections::HashMap;
use std::net::Ipv4Addr;

use aws_config::{self, meta::region::RegionProviderChain, SdkConfig};
use aws_sdk_route53 as route53;
use route53::{
    config::Region,
    types::{
        builders::{ChangeBuilder, ResourceRecordBuilder, ResourceRecordSetBuilder},
        ChangeBatch,
    },
};

mod logger;

use logger::DbLogger;

fn get_data() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("ZY20L3SG16SCT", "jonathansamson.com");
    map.insert("Z07550552NG0KQSGPGT8L", "stinkrs.com");
    map
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::builder()
        .target(env_logger::Target::Stdout)
        .init();

    run().await
}

use env_logger;
use log::*;

use crate::logger::DomainIpSyncRow;

pub async fn run() -> Result<()> {
    debug!("start");
    info!("start!");

    let config = get_aws_config().await;
    let client = route53::Client::new(&config);

    let ip = public_ip::addr_v4()
        .await
        .context("Error getting public ip.")?;
    debug!("Server address is {}", ip);

    for (hosted_zone_id, domain) in get_data().iter() {
        debug!("{hosted_zone_id}, {domain}");

        DomainIpSyncRow {
            id: 0,
            hosted_zone_id: hosted_zone_id.to_string(),
            domain: domain.to_string(),
            ip_address: ip.to_string(),
            timestamp: None,
        }
        .gen_log()
        .await?;

        debug!("Processing hosted zone {hosted_zone_id}");
        let a_record = get_a_record(hosted_zone_id, domain, &client).await?;
        debug!("A record is {:?}", a_record);

        if a_record == ip.to_string() {
            debug!("IP is already up to date.");
            continue;
        }

        update_dns(&client, hosted_zone_id, domain, ip).await?;
        gen_log(hosted_zone_id, domain, ip).await?;
    }

    Ok(())
}

async fn gen_log(hosted_zone_id: &str, domain: &str, ip: Ipv4Addr) -> Result<()> {
    info!("Updated A record of {hosted_zone_id} to {ip}");
    logger::DomainIpSyncRow {
        id: 0,
        hosted_zone_id: hosted_zone_id.to_string(),
        domain: domain.to_string(),
        ip_address: ip.to_string(),
        timestamp: None,
    }
    .gen_log()
    .await?;
    Ok(())
}

async fn get_aws_config() -> SdkConfig {
    let region_provider = RegionProviderChain::first_try(Region::new("us-east-1"))
        .or_default_provider()
        .or_else(Region::new("us-west-2"));

    aws_config::from_env().region(region_provider).load().await
}

async fn get_a_record(
    hosted_zone_id: &str,
    record_name: &str,
    client: &route53::Client,
) -> Result<String> {
    let request = client
        .list_resource_record_sets()
        .hosted_zone_id(hosted_zone_id)
        .start_record_name(record_name)
        .start_record_type(route53::types::RrType::A)
        .max_items(1)
        .send()
        .await?;

    let record_sets = request
        .resource_record_sets
        .context("Failed to fetch resource record sets")?;

    let resource_record = record_sets
        .first()
        .and_then(|record_set| record_set.resource_records.as_ref())
        .and_then(|records| records.first())
        .and_then(|record| record.value.as_ref())
        .map(String::clone)
        .context("No resource records present")?;

    Ok(resource_record)
}

async fn update_dns(
    client: &route53::Client,
    hosted_zone_id: &str, // HOSTED_ZONE_ID
    record_name: &str,    // jonathansamson.com
    ip: Ipv4Addr,
) -> Result<()> {
    let resource_record_set = ResourceRecordSetBuilder::default()
        .name(record_name)
        .r#type(route53::types::RrType::A)
        .resource_records(
            ResourceRecordBuilder::default()
                .value(ip.to_string())
                .build(),
        )
        .ttl(300)
        .build();
    let change = ChangeBuilder::default()
        .resource_record_set(resource_record_set)
        .action(route53::types::ChangeAction::Upsert)
        .build();
    let change_batch = ChangeBatch::builder().changes(change).build();
    client
        .change_resource_record_sets()
        .hosted_zone_id(hosted_zone_id)
        .change_batch(change_batch)
        .send()
        .await?;
    Ok(())
}
