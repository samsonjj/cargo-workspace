use std::net::Ipv4Addr;

use aws_config::{self, meta::region::RegionProviderChain, SdkConfig};
use aws_sdk_route53 as route53;
use public_ip;
use route53::{
    config::Region,
    types::{
        builders::{ChangeBuilder, ResourceRecordBuilder, ResourceRecordSetBuilder},
        ChangeBatch,
    },
};
use tokio;

const HOSTED_ZONE_ID: &str = "ZY20L3SG16SCT";

#[tokio::main]
async fn main() -> Result<(), &'static str> {
    let config = get_aws_config().await;
    let client = route53::Client::new(&config);

    let ip = public_ip::addr_v4()
        .await
        .ok_or("Error getting public ip.")?;
    println!("Server address is {}", ip.to_string());

    let a_record = get_a_record(&client).await;
    println!("A record is {:?}", a_record);

    if a_record == Some(ip.to_string()) {
        println!("IP is already up to date.");
        return Ok(());
    }

    update_dns(&client, ip).await;
    println!("Updated A record to {}", ip.to_string());

    Ok(())
}

async fn get_aws_config() -> SdkConfig {
    let region_provider = RegionProviderChain::first_try(Region::new("us-east-1"))
        .or_default_provider()
        .or_else(Region::new("us-west-2"));
    let config = aws_config::from_env().region(region_provider).load().await;
    config
}

async fn get_a_record(client: &route53::Client) -> Option<String> {
    let record_sets = client
        .list_resource_record_sets()
        .hosted_zone_id(HOSTED_ZONE_ID)
        .max_items(1)
        .send()
        .await
        .unwrap()
        .resource_record_sets;
    let resource_record = record_sets?
        .first()?
        .resource_records
        .as_ref()?
        .first()?
        .value()?
        .to_string();
    Some(resource_record)
}

async fn update_dns(client: &route53::Client, ip: Ipv4Addr) {
    let resource_record_set = ResourceRecordSetBuilder::default()
        .name("jonathansamson.com")
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
        .hosted_zone_id(HOSTED_ZONE_ID)
        .change_batch(change_batch)
        .send()
        .await
        .unwrap();
}
