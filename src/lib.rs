pub mod lh_models;

use bson::oid::ObjectId;
use getset::{Getters, Setters};
use lh_models::{
    Audit, AuditSimple, AuditTable, Filmstrip, LatencyItem, NetworkRequest, NetworkRttItem, Node,
    Opportunity, Resource, ScriptExecutionItem, ThirdPartyDetail, Throttling, UrlProtocol,
    WorkBreakdownItem,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct ScoreParameters {
    pub page: Option<PageScoreParameters>,
    pub site: Option<SiteScoreParameters>,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct PageScoreParameters {
    pub url: String,
    pub throttling: Option<String>,
    pub attempts: Option<i8>,
    pub device: Option<String>,

    #[serde(rename = "lighthouseVersion")]
    pub lighthouse_version: Option<String>,

    #[serde(rename = "blockedUrlPatterns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_url_patterns: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct SiteScoreParameters {
    #[serde(rename = "siteId")]
    pub site_id: String,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct AuditDetail {
    #[serde(rename = "lighthouseVersion")]
    lighthouse_version: String,

    #[serde(rename = "requestedUrl")]
    requested_url: String,

    #[serde(rename = "finalUrl")]
    final_url: String,

    #[serde(rename = "fetchTime")]
    fetch_time: String,

    #[serde(rename = "categories")]
    categories: Categories,

    #[serde(rename = "configSettings")]
    config_settings: ConfigSettings,

    #[serde(rename = "webVitals")]
    web_vitals: WebVitals,

    #[serde(rename = "largestContentfulPaintElement")]
    largest_contentful_paint_element: Option<AuditTable<Node>>,

    #[serde(rename = "networkRequests")]
    network_requests: Option<AuditTable<NetworkRequest>>,

    #[serde(rename = "resourceSummary")]
    resource_summary: Option<AuditTable<Resource>>,

    #[serde(rename = "thirdPartySummary")]
    third_party_summary: Option<AuditTable<ThirdPartyDetail>>,

    #[serde(rename = "screenshotThumbnails")]
    screenshot_thumbnails: Option<Audit<Filmstrip>>,

    #[serde(rename = "usesResponsiveImages")]
    uses_responsive_images: Option<Audit<Opportunity>>,

    #[serde(rename = "usesOptimizedImages")]
    uses_optimized_images: Option<Audit<Opportunity>>,

    #[serde(rename = "usesWebpImages")]
    uses_webp_images: Option<Audit<Opportunity>>,

    #[serde(rename = "offscreenImages")]
    offscreen_images: Option<Audit<Opportunity>>,

    #[serde(rename = "usesHttp2")]
    uses_http2: Option<AuditTable<UrlProtocol>>,

    #[serde(rename = "bootupTime")]
    bootup_time: Option<AuditTable<ScriptExecutionItem>>,

    #[serde(rename = "mainThreadWorkBreakdown")]
    main_thread_work_breakdown: Option<AuditTable<WorkBreakdownItem>>,

    #[serde(rename = "usesRelPreconnect")]
    uses_rel_preconnect: Option<AuditSimple>,

    #[serde(rename = "networkServerLatency")]
    network_server_latency: Option<AuditTable<LatencyItem>>,

    #[serde(rename = "networkRtt")]
    network_rtt: Option<AuditTable<NetworkRttItem>>,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct WebVitals {
    #[serde(rename = "firstContentfulPaint")]
    first_contentful_paint: AuditSimple,

    #[serde(rename = "speedIndex")]
    speed_index: AuditSimple,

    #[serde(rename = "largestContentfulPaint")]
    largest_contentful_paint: Option<AuditSimple>,

    #[serde(rename = "interactive")]
    interactive: AuditSimple,

    #[serde(rename = "totalBlockingTime")]
    total_blocking_time: AuditSimple,

    #[serde(rename = "cumulativeLayoutShift")]
    cumulative_layout_shift: Option<AuditSimple>,

    //LH5 metrics; but not in LH6
    #[serde(rename = "maxPotentialFid")]
    max_potential_fid: AuditSimple,
    #[serde(rename = "firstMeaningfulPaint")]
    first_meaningful_paint: AuditSimple,

    #[serde(rename = "firstCpuIdle")]
    first_cpu_idle: AuditSimple,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct Categories {
    #[serde(rename = "performance")]
    performance: Performance,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct Performance {
    id: String,
    title: String,
    score: f64,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct ConfigSettings {
    #[serde(rename = "throttlingMethod")]
    throttling_method: String,

    #[serde(rename = "throttling")]
    throttling: Throttling,

    #[serde(rename = "emulatedFormFactor")]
    emulated_form_factor: String,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct LighthouseSettings {
    #[serde(rename = "devices")]
    devices: Vec<String>,

    #[serde(rename = "versions")]
    versions: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct Site {
    #[serde(rename = "_id")]
    id: ObjectId,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "pages")]
    pages: Vec<Page>,

    #[serde(rename = "auditProfiles")]
    audit_profiles: Vec<AuditProfile>,

    #[serde(rename = "lighthouseSettings")]
    lighthouse_settings: LighthouseSettings,

    #[serde(rename = "lastRunId")]
    last_run_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct Page {
    #[serde(rename = "id")]
    id: String,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "url")]
    url: String,
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct AuditProfile {
    #[serde(rename = "id")]
    id: String,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "device")]
    device: String,

    #[serde(rename = "lighthouseVersion")]
    lighthouse_version: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "enabled")]
    enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "blockedUrlPatterns")]
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
pub struct AuditSummary {
    #[serde(rename = "siteId")]
    site_id: ObjectId,

    #[serde(rename = "siteRunId")]
    site_run_id: i32,

    #[serde(rename = "pageId")]
    page_id: String,

    #[serde(rename = "auditProfileId")]
    audit_profile_id: String,

    #[serde(rename = "auditProfile")]
    audit_profile: AuditProfile,

    #[serde(rename = "fetchTime")]
    fetch_time: String,

    #[serde(rename = "categories")]
    categories: Categories,

    #[serde(rename = "configSettings")]
    config_settings: ConfigSettings,

    #[serde(rename = "webVitals")]
    web_vitals: WebVitals,

    #[serde(rename = "auditDetailId")]
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
pub struct AuditSummaryOld {
    #[serde(rename = "pageName")]
    page_name: String,

    #[serde(rename = "auditProfile")]
    audit_profile: AuditProfile,

    #[serde(rename = "lighthouseVersion")]
    lighthouse_version: String,

    #[serde(rename = "requestedUrl")]
    requested_url: String,

    #[serde(rename = "finalUrl")]
    final_url: String,

    #[serde(rename = "fetchTime")]
    fetch_time: String,

    #[serde(rename = "categories")]
    categories: Categories,

    #[serde(rename = "configSettings")]
    config_settings: ConfigSettings,

    #[serde(rename = "webVitals")]
    web_vitals: WebVitals,

    #[serde(rename = "auditDetailId")]
    audit_detail_id: ObjectId,
}

impl AuditSummaryOld {
    pub fn new(
        page_name: String,
        audit_profile: AuditProfile,
        lighthouse_version: String,
        requested_url: String,
        final_url: String,
        fetch_time: String,
        categories: Categories,
        config_settings: ConfigSettings,
        web_vitals: WebVitals,
        audit_detail_id: ObjectId,
    ) -> AuditSummaryOld {
        AuditSummaryOld {
            page_name,
            audit_profile,
            lighthouse_version,
            requested_url,
            final_url,
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
pub struct PageAuditSummary {
    #[serde(rename = "siteId")]
    site_id: ObjectId,

    #[serde(rename = "pageName")]
    page_name: String,

    #[serde(rename = "url")]
    url: String,

    #[serde(rename = "auditSummaries")]
    audit_summaries: HashMap<String, AuditSummaryOld>,
}

impl PageAuditSummary {
    pub fn new(site_id: ObjectId, page_name: String, url: String) -> PageAuditSummary {
        PageAuditSummary {
            site_id,
            page_name,
            url,
            audit_summaries: HashMap::new(),
        }
    }

    pub fn add_audit_summary(&mut self, audit_summary: AuditSummaryOld) {
        let audit_profile_name = format!(
            "{}-{}",
            audit_summary.audit_profile.device, audit_summary.lighthouse_version
        );
        self.audit_summaries
            .insert(audit_profile_name, audit_summary);
    }
}

#[derive(Deserialize, Serialize, Debug, Getters, Setters, Default, Clone)]
#[getset(get = "pub", set = "pub")]
pub struct SiteTread {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_id")]
    id: Option<ObjectId>,

    #[serde(rename = "siteId")]
    site_id: ObjectId,

    #[serde(rename = "siteName")]
    site_name: String,

    #[serde(rename = "pageAuditSummaries")]
    page_audit_summaries: Vec<PageAuditSummary>,
}

impl SiteTread {
    pub fn new(site_id: ObjectId, site_name: String) -> SiteTread {
        SiteTread {
            id: None,
            site_id,
            site_name,
            page_audit_summaries: Vec::new(),
        }
    }

    pub fn add_page_audit_summary(&mut self, page_audit_summary: PageAuditSummary) {
        self.page_audit_summaries.push(page_audit_summary);
    }
}
