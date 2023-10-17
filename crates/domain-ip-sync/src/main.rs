use std::net::Ipv4Addr;

use aws_config::{self, meta::region::RegionProviderChain, SdkConfig};
use aws_sdk_route53 as route53;
use const_singleton::ConstSingleton;
use route53::{
    config::Region,
    types::{
        builders::{ChangeBuilder, ResourceRecordBuilder, ResourceRecordSetBuilder},
        ChangeBatch,
    },
};

use lazy_static::lazy_static;
lazy_static! {
    static ref HOSTED_ZONE_IDS: Vec<&'static str> = vec!["ZY20L3SG16SCT", "Z07550552NG0KQSGPGT8L"];
    static ref EXAMPLE: u8 = 42;
}

#[tokio::main]
async fn main() {
    env_logger::builder()
        .target(env_logger::Target::Stdout)
        .init();

    let result = run().await;
    if let Err(e) = result {
        error!("Error: {}", e);
    }
}

use env_logger;
use log::*;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = get_aws_config().await;
    let client = route53::Client::new(&config);

    let ip = public_ip::addr_v4()
        .await
        .ok_or("Error getting public ip.")?;
    debug!("Server address is {}", ip);

    for hosted_zone_id in HOSTED_ZONE_IDS.iter() {
        debug!("Processing hosted zone {hosted_zone_id}");
        let a_record = get_a_record(hosted_zone_id, &client).await?;
        debug!("A record is {:?}", a_record);

        if a_record == ip.to_string() {
            debug!("IP is already up to date.");
            continue;
        }

        update_dns(&client, hosted_zone_id, "jonathansamson.com", ip).await?;
        info!("Updated A record of {hosted_zone_id} to {ip}");
    }

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
    client: &route53::Client,
) -> Result<String, Box<dyn std::error::Error>> {
    let request = client
        .list_resource_record_sets()
        .hosted_zone_id(hosted_zone_id)
        .start_record_name("jonathansamson.com")
        .start_record_type(route53::types::RrType::A)
        .max_items(1)
        .send()
        .await?;

    let record_sets = request
        .resource_record_sets
        .ok_or("Failed to fetch resource record sets")?;

    let resource_record = record_sets
        .first()
        .and_then(|record_set| record_set.resource_records.as_ref())
        .and_then(|records| records.first())
        .and_then(|record| record.value.as_ref())
        .map(String::clone)
        .ok_or("No resource records present")?;

    Ok(resource_record)
}

async fn update_dns(
    client: &route53::Client,
    hosted_zone_id: &str, // HOSTED_ZONE_ID
    record_name: &str,    // jonathansamson.com
    ip: Ipv4Addr,
) -> Result<(), Box<dyn std::error::Error>> {
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
