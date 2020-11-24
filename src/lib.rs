pub mod lh_models;

use bson::oid::ObjectId;
use getset::{Getters, Setters};
use lh_models::{
    Audit, AuditSimple, AuditTable, CachePolicyItem, Filmstrip, LatencyItem, NetworkRequest,
    NetworkRttItem, Node, Opportunity, Resource, ScriptExecutionItem, Task, ThirdPartyDetail,
    Throttling, UserTiming, WorkBreakdownItem,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct ScoreParameters {
    pub page: Option<PageScoreParameters>,
    pub site: Option<SiteScoreParameters>,
}

impl ScoreParameters {
    pub fn for_page(page: PageScoreParameters) -> ScoreParameters {
        ScoreParameters {
            page: Some(page),
            site: None,
        }
    }

    pub fn for_site(site: SiteScoreParameters) -> ScoreParameters {
        ScoreParameters {
            page: None,
            site: Some(site),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
#[serde(rename_all = "camelCase")]
pub struct PageScoreParameters {
    pub url: String,
    pub throttling: Option<String>,
    pub attempts: Option<i8>,
    pub device: Option<String>,
    pub lighthouse_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_url_patterns: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie: Option<Cookie>,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
#[serde(rename_all = "camelCase")]
pub struct SiteScoreParameters {
    pub site_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie: Option<Cookie>,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
#[serde(rename_all = "camelCase")]
pub struct AuditDetail {
    lighthouse_version: String,
    requested_url: String,
    final_url: String,
    fetch_time: String,
    categories: Categories,
    config_settings: ConfigSettings,
    web_vitals: WebVitals,
    largest_contentful_paint_element: Option<AuditTable<Node>>,
    network_requests: Option<AuditTable<NetworkRequest>>,
    resource_summary: Option<AuditTable<Resource>>,
    third_party_summary: Option<AuditTable<ThirdPartyDetail>>,
    screenshot_thumbnails: Option<Audit<Filmstrip>>,
    uses_responsive_images: Option<Audit<Opportunity>>,
    uses_optimized_images: Option<Audit<Opportunity>>,
    uses_webp_images: Option<Audit<Opportunity>>,
    offscreen_images: Option<Audit<Opportunity>>,
    uses_http2: Option<Audit<Opportunity>>,
    bootup_time: Option<AuditTable<ScriptExecutionItem>>,
    main_thread_work_breakdown: Option<AuditTable<WorkBreakdownItem>>,
    uses_rel_preconnect: Option<AuditSimple>,
    network_server_latency: Option<AuditTable<LatencyItem>>,
    network_rtt: Option<AuditTable<NetworkRttItem>>,
    main_thread_tasks: Option<AuditTable<Task>>,
    unminified_css: Option<Audit<Opportunity>>,
    unminified_javascript: Option<Audit<Opportunity>>,
    unused_css_rules: Option<Audit<Opportunity>>,
    unused_javascript: Option<Audit<Opportunity>>,
    render_blocking_resources: Option<Audit<Opportunity>>,
    uses_long_cache_ttl: Option<AuditTable<CachePolicyItem>>,
    user_timings: Option<AuditTable<UserTiming>>,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
#[serde(rename_all = "camelCase")]
pub struct WebVitals {
    first_contentful_paint: AuditSimple,
    speed_index: AuditSimple,
    largest_contentful_paint: Option<AuditSimple>,
    interactive: AuditSimple,
    total_blocking_time: AuditSimple,
    cumulative_layout_shift: Option<AuditSimple>,

    //LH5 metrics; but not in LH6
    max_potential_fid: AuditSimple,
    first_meaningful_paint: AuditSimple,
    first_cpu_idle: AuditSimple,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct Categories {
    performance: Performance,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
#[serde(rename_all = "camelCase")]
pub struct Performance {
    id: String,
    title: String,
    score: f64,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
#[serde(rename_all = "camelCase")]
pub struct ConfigSettings {
    throttling_method: String,
    throttling: Throttling,
    emulated_form_factor: String,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct LighthouseSettings {
    devices: Vec<String>,
    versions: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
#[serde(rename_all = "camelCase")]
pub struct Site {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_id")]
    id: Option<ObjectId>,
    name: String,
    group_id: Option<ObjectId>,
    pages: Vec<Page>,
    audit_profiles: Vec<AuditProfile>,
    last_run_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    authentication: Option<Authentication>,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
#[serde(rename_all = "camelCase")]
pub struct MetaSite {
    #[serde(rename = "_id")]
    id: ObjectId,
    name: String,
    group_id: Option<ObjectId>,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Clone)]
#[getset(get = "pub", set = "pub")]
#[serde(rename_all = "camelCase")]
pub struct Authentication {
    #[serde(rename = "type")]
    authentication_type: AuthenticationType,
    cookie: Option<Cookie>,
}

impl Authentication {
    pub fn from_cookie(cookie: Cookie) -> Authentication {
        Authentication {
            authentication_type: AuthenticationType::Cookie,
            cookie: Some(cookie),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum AuthenticationType {
    Cookie,
    None,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
#[serde(rename_all = "camelCase")]
pub struct Page {
    #[serde(rename = "id")]
    id: String,
    name: String,
    url: String,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
#[serde(rename_all = "camelCase")]
pub struct AuditProfile {
    #[serde(rename = "id")]
    id: String,
    name: String,
    device: String,
    lighthouse_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    blocked_url_patterns: Option<Vec<String>>,
}

impl AuditProfile {
    pub fn new(
        id: String,
        name: String,
        device: String,
        lighthouse_version: String,
    ) -> AuditProfile {
        AuditProfile {
            id,
            name,
            device,
            lighthouse_version,
            enabled: None,
            blocked_url_patterns: None,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
#[serde(rename_all = "camelCase")]
pub struct AuditSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_id")]
    id: Option<ObjectId>,
    site_id: ObjectId,
    site_run_id: i32,
    page_id: String,
    audit_profile_id: String,
    audit_profile: AuditProfile,
    fetch_time: String,
    categories: Categories,
    config_settings: ConfigSettings,
    web_vitals: WebVitals,
    audit_detail_id: ObjectId,
}

impl AuditSummary {
    pub fn new(
        site_id: ObjectId,
        site_run_id: i32,
        page_id: String,
        audit_profile_id: String,
        audit_profile: AuditProfile,
        fetch_time: String,
        categories: Categories,
        config_settings: ConfigSettings,
        web_vitals: WebVitals,
        audit_detail_id: ObjectId,
    ) -> AuditSummary {
        AuditSummary {
            id: None,
            site_id,
            site_run_id,
            page_id,
            audit_profile_id,
            audit_profile,
            fetch_time,
            categories,
            config_settings,
            web_vitals,
            audit_detail_id,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct Cookie {
    name: String,
    value: String,
}

impl Cookie {
    pub fn new(name: String, value: String) -> Cookie {
        Cookie { name, value }
    }
}
