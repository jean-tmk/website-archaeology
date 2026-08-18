//! Website Archaeology — excavation and conservation engine.
//!
//! This crate models an interface as a stratigraphic site. The public API is
//! deliberately independent from the renderer: a browser, terminal, or museum
//! installation can feed brush samples into the same excavation state machine.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use wasm_bindgen::prelude::*;

pub const GRID_COLUMNS: usize = 48;
pub const GRID_ROWS: usize = 30;
pub const CELL_COUNT: usize = GRID_COLUMNS * GRID_ROWS;
pub const DESCENT_THRESHOLD: f32 = 0.68;
pub const CATALOGUE_SIZE: usize = 12;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Era {
    OptimizedPresent,
    ResponsiveTurn,
    PersonalWeb,
    HandmadeSource,
}

impl Era {
    pub const ALL: [Era; 4] = [
        Era::OptimizedPresent,
        Era::ResponsiveTurn,
        Era::PersonalWeb,
        Era::HandmadeSource,
    ];

    pub fn index(self) -> usize {
        match self {
            Era::OptimizedPresent => 0,
            Era::ResponsiveTurn => 1,
            Era::PersonalWeb => 2,
            Era::HandmadeSource => 3,
        }
    }

    pub fn year(self) -> u16 {
        match self {
            Era::OptimizedPresent => 2026,
            Era::ResponsiveTurn => 2012,
            Era::PersonalWeb => 2003,
            Era::HandmadeSource => 1996,
        }
    }

    pub fn depth_cm(self) -> f32 {
        match self {
            Era::OptimizedPresent => 0.0,
            Era::ResponsiveTurn => 14.8,
            Era::PersonalWeb => 31.6,
            Era::HandmadeSource => 52.4,
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Era::OptimizedPresent => "The Optimized Present",
            Era::ResponsiveTurn => "The Responsive Turn",
            Era::PersonalWeb => "The Personal Web",
            Era::HandmadeSource => "The Handmade Source",
        }
    }

    pub fn field_note(self) -> &'static str {
        match self {
            Era::OptimizedPresent => {
                "The newest layer is usually the loudest. It is rarely the deepest."
            }
            Era::ResponsiveTurn => {
                "This stratum believed every problem could be solved with a grid, a gradient, and three equal columns."
            }
            Era::PersonalWeb => {
                "Before platforms became places, a homepage was a room someone decorated and left unlocked."
            }
            Era::HandmadeSource => {
                "At bedrock, design and code are the same gesture. View Source is both instruction and invitation."
            }
        }
    }

    pub fn next(self) -> Option<Era> {
        Era::ALL.get(self.index() + 1).copied()
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Material {
    Glass,
    Metric,
    GeneratedText,
    Navigation,
    Carousel,
    SocialProof,
    Guestbook,
    ConstructionGif,
    Midi,
    TableLayout,
    Webring,
    SourceCode,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Artifact {
    pub catalogue_number: u8,
    pub era: Era,
    pub material: Material,
    pub name: &'static str,
    pub description: &'static str,
    pub significance: &'static str,
    pub rarity: u8,
    pub discovery_threshold: f32,
    pub grid_hint: (usize, usize),
}

pub const ARTIFACTS: [Artifact; CATALOGUE_SIZE] = [
    Artifact {
        catalogue_number: 1,
        era: Era::OptimizedPresent,
        material: Material::Glass,
        name: "Glass Button",
        description: "Polished until the label disappeared",
        significance: "A surviving control from the age when frictionless surfaces concealed increasingly complicated systems.",
        rarity: 2,
        discovery_threshold: 0.14,
        grid_hint: (9, 8),
    },
    Artifact {
        catalogue_number: 2,
        era: Era::OptimizedPresent,
        material: Material::Metric,
        name: "Engagement Pill",
        description: "Designed to contain one metric",
        significance: "Displayed an abstract measurement as if it were a physical nutrient the visitor could consume.",
        rarity: 2,
        discovery_threshold: 0.35,
        grid_hint: (35, 6),
    },
    Artifact {
        catalogue_number: 3,
        era: Era::OptimizedPresent,
        material: Material::GeneratedText,
        name: "AI Summary",
        description: "Confidently compressed from six tabs",
        significance: "A compact textual layer deposited above sources that were no longer expected to be visited.",
        rarity: 3,
        discovery_threshold: 0.61,
        grid_hint: (23, 22),
    },
    Artifact {
        catalogue_number: 4,
        era: Era::ResponsiveTurn,
        material: Material::Navigation,
        name: "Hamburger Icon",
        description: "Navigation folded into three lines",
        significance: "Evidence of the Great Screen Negotiation, when every interface learned to collapse itself.",
        rarity: 1,
        discovery_threshold: 0.14,
        grid_hint: (41, 3),
    },
    Artifact {
        catalogue_number: 5,
        era: Era::ResponsiveTurn,
        material: Material::Carousel,
        name: "Hero Carousel",
        description: "Four messages, none remembered",
        significance: "A rotating monument to unresolved stakeholder priority, often found above three equal columns.",
        rarity: 2,
        discovery_threshold: 0.35,
        grid_hint: (22, 11),
    },
    Artifact {
        catalogue_number: 6,
        era: Era::ResponsiveTurn,
        material: Material::SocialProof,
        name: "Social Counter",
        description: "Proof that 2,481 people had clicked",
        significance: "A numerical charm believed to increase trust through the public display of previous attention.",
        rarity: 2,
        discovery_threshold: 0.61,
        grid_hint: (8, 24),
    },
    Artifact {
        catalogue_number: 7,
        era: Era::PersonalWeb,
        material: Material::Guestbook,
        name: "Guestbook Entry",
        description: "hi cool site sign mine pls",
        significance: "An early social trace proving that visitors once introduced themselves before leaving a page.",
        rarity: 4,
        discovery_threshold: 0.14,
        grid_hint: (31, 25),
    },
    Artifact {
        catalogue_number: 8,
        era: Era::PersonalWeb,
        material: Material::ConstructionGif,
        name: "Under Construction",
        description: "A promise rendered as a tiny worker",
        significance: "A declaration that a website was a living place rather than a finished publication.",
        rarity: 3,
        discovery_threshold: 0.35,
        grid_hint: (13, 18),
    },
    Artifact {
        catalogue_number: 9,
        era: Era::PersonalWeb,
        material: Material::Midi,
        name: "MIDI Autoplay",
        description: "The sound of arriving unannounced",
        significance: "A compressed musical greeting that transformed navigation into an involuntary performance.",
        rarity: 5,
        discovery_threshold: 0.61,
        grid_hint: (39, 9),
    },
    Artifact {
        catalogue_number: 10,
        era: Era::HandmadeSource,
        material: Material::TableLayout,
        name: "Table Layout",
        description: "The first dependable architecture",
        significance: "Tabular data repurposed as shelter during the period before dedicated layout systems.",
        rarity: 4,
        discovery_threshold: 0.14,
        grid_hint: (24, 15),
    },
    Artifact {
        catalogue_number: 11,
        era: Era::HandmadeSource,
        material: Material::Webring,
        name: "Webring Shard",
        description: "A door to somebody else's page",
        significance: "A fragment of a human-curated path connecting independent pages without a ranking algorithm.",
        rarity: 5,
        discovery_threshold: 0.35,
        grid_hint: (5, 27),
    },
    Artifact {
        catalogue_number: 12,
        era: Era::HandmadeSource,
        material: Material::SourceCode,
        name: "View Source",
        description: "The oldest surviving tutorial",
        significance: "A browser affordance through which the web taught newcomers how to build more web.",
        rarity: 5,
        discovery_threshold: 0.61,
        grid_hint: (43, 21),
    },
];

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct Cell {
    pub exposure: u8,
    pub passes: u16,
    pub damaged: bool,
    pub last_tool_radius: u8,
}

impl Cell {
    pub fn is_exposed(self) -> bool {
        self.exposure >= 220
    }

    pub fn is_disturbed(self) -> bool {
        self.exposure > 0
    }

    pub fn apply_pressure(&mut self, pressure: f32, tool_radius: u8) -> u8 {
        let normalized = pressure.clamp(0.0, 1.0);
        let removal = (normalized * 46.0).round() as u8;
        self.exposure = self.exposure.saturating_add(removal);
        self.passes = self.passes.saturating_add(1);
        self.last_tool_radius = tool_radius;
        if normalized > 0.92 && self.passes > 5 {
            self.damaged = true;
        }
        removal
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct BrushSample {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub pressure: f32,
    pub timestamp_ms: u64,
}

impl BrushSample {
    pub fn normalized(self) -> Self {
        Self {
            x: self.x.clamp(0.0, 1.0),
            y: self.y.clamp(0.0, 1.0),
            radius: self.radius.clamp(0.005, 0.25),
            pressure: self.pressure.clamp(0.05, 1.0),
            timestamp_ms: self.timestamp_ms,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FindRecord {
    pub catalogue_number: u8,
    pub era: Era,
    pub discovered_at_ms: u64,
    pub exposure_at_discovery: f32,
    pub context_preserved: bool,
    pub field_coordinates: (usize, usize),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LayerReport {
    pub era: Era,
    pub exposure: f32,
    pub disturbed_cells: usize,
    pub exposed_cells: usize,
    pub damaged_cells: usize,
    pub artifact_count: usize,
    pub conservation_score: u8,
    pub may_descend: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FieldEvent {
    LayerOpened { era: Era },
    SurfaceDisturbed { cells: usize },
    ArtifactRecovered { catalogue_number: u8 },
    ThresholdReached { era: Era, exposure: f32 },
    LayerClosed { report: LayerReport },
    BedrockReached,
    SiteReset,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExcavationLayer {
    pub era: Era,
    pub cells: Vec<Cell>,
    pub finds: BTreeSet<u8>,
    pub started_at_ms: u64,
    pub completed_at_ms: Option<u64>,
    samples: VecDeque<BrushSample>,
}

impl ExcavationLayer {
    pub fn new(era: Era, started_at_ms: u64) -> Self {
        Self {
            era,
            cells: vec![Cell::default(); CELL_COUNT],
            finds: BTreeSet::new(),
            started_at_ms,
            completed_at_ms: None,
            samples: VecDeque::with_capacity(256),
        }
    }

    pub fn reset(&mut self, timestamp_ms: u64) {
        self.cells.fill(Cell::default());
        self.finds.clear();
        self.samples.clear();
        self.started_at_ms = timestamp_ms;
        self.completed_at_ms = None;
    }

    pub fn exposure(&self) -> f32 {
        let total: u64 = self.cells.iter().map(|cell| cell.exposure as u64).sum();
        total as f32 / (CELL_COUNT as f32 * u8::MAX as f32)
    }

    pub fn exposed_cell_ratio(&self) -> f32 {
        let exposed = self.cells.iter().filter(|cell| cell.is_exposed()).count();
        exposed as f32 / CELL_COUNT as f32
    }

    pub fn disturbed_cells(&self) -> usize {
        self.cells.iter().filter(|cell| cell.is_disturbed()).count()
    }

    pub fn damaged_cells(&self) -> usize {
        self.cells.iter().filter(|cell| cell.damaged).count()
    }

    pub fn conservation_score(&self) -> u8 {
        let damage_ratio = self.damaged_cells() as f32 / CELL_COUNT as f32;
        let repeat_pressure = self
            .cells
            .iter()
            .filter(|cell| cell.passes > 12)
            .count() as f32
            / CELL_COUNT as f32;
        let artifact_bonus = self.finds.len() as f32 / 3.0 * 10.0;
        let score = 100.0 - damage_ratio * 90.0 - repeat_pressure * 35.0 + artifact_bonus;
        score.clamp(0.0, 100.0).round() as u8
    }

    pub fn report(&self) -> LayerReport {
        let exposure = self.exposure();
        LayerReport {
            era: self.era,
            exposure,
            disturbed_cells: self.disturbed_cells(),
            exposed_cells: self.cells.iter().filter(|cell| cell.is_exposed()).count(),
            damaged_cells: self.damaged_cells(),
            artifact_count: self.finds.len(),
            conservation_score: self.conservation_score(),
            may_descend: exposure >= DESCENT_THRESHOLD,
        }
    }

    pub fn apply_sample(&mut self, sample: BrushSample) -> usize {
        let sample = sample.normalized();
        self.samples.push_back(sample);
        if self.samples.len() > 240 {
            self.samples.pop_front();
        }
        let center_x = sample.x * GRID_COLUMNS as f32;
        let center_y = sample.y * GRID_ROWS as f32;
        let radius_x = (sample.radius * GRID_COLUMNS as f32).max(1.0);
        let radius_y = (sample.radius * GRID_ROWS as f32).max(1.0);
        let min_x = (center_x - radius_x).floor().max(0.0) as usize;
        let max_x = (center_x + radius_x)
            .ceil()
            .min((GRID_COLUMNS - 1) as f32) as usize;
        let min_y = (center_y - radius_y).floor().max(0.0) as usize;
        let max_y = (center_y + radius_y)
            .ceil()
            .min((GRID_ROWS - 1) as f32) as usize;
        let mut disturbed = 0;
        let tool_radius = (sample.radius * 255.0).round() as u8;

        for row in min_y..=max_y {
            for column in min_x..=max_x {
                let dx = (column as f32 + 0.5 - center_x) / radius_x;
                let dy = (row as f32 + 0.5 - center_y) / radius_y;
                let distance = (dx * dx + dy * dy).sqrt();
                if distance <= 1.0 {
                    let feather = (1.0 - distance).powf(1.6);
                    let pressure = sample.pressure * (0.25 + feather * 0.75);
                    let index = row * GRID_COLUMNS + column;
                    if self.cells[index].apply_pressure(pressure, tool_radius) > 0 {
                        disturbed += 1;
                    }
                }
            }
        }
        disturbed
    }

    pub fn survey_sweep(&mut self, seed: u64, timestamp_ms: u64) -> usize {
        let lane = pseudo_random(seed) * 0.56 + 0.22;
        let phase = pseudo_random(seed.rotate_left(13)) * std::f32::consts::TAU;
        let mut affected = 0;
        for step in 0..30 {
            let t = step as f32 / 29.0;
            let y = lane + (t * std::f32::consts::TAU + phase).sin() * 0.055;
            affected += self.apply_sample(BrushSample {
                x: t,
                y,
                radius: 0.08,
                pressure: 0.72,
                timestamp_ms: timestamp_ms + step as u64 * 16,
            });
        }
        affected
    }

    pub fn eligible_artifacts(&self) -> Vec<&'static Artifact> {
        let exposure = self.exposure();
        ARTIFACTS
            .iter()
            .filter(|artifact| artifact.era == self.era)
            .filter(|artifact| artifact.discovery_threshold <= exposure)
            .filter(|artifact| !self.finds.contains(&artifact.catalogue_number))
            .collect()
    }

    pub fn recover_eligible(&mut self, timestamp_ms: u64) -> Vec<FindRecord> {
        let exposure = self.exposure();
        let eligible: Vec<Artifact> = self.eligible_artifacts().into_iter().cloned().collect();
        eligible
            .into_iter()
            .map(|artifact| {
                self.finds.insert(artifact.catalogue_number);
                let index = artifact.grid_hint.1 * GRID_COLUMNS + artifact.grid_hint.0;
                FindRecord {
                    catalogue_number: artifact.catalogue_number,
                    era: artifact.era,
                    discovered_at_ms: timestamp_ms,
                    exposure_at_discovery: exposure,
                    context_preserved: !self.cells[index.min(CELL_COUNT - 1)].damaged,
                    field_coordinates: artifact.grid_hint,
                }
            })
            .collect()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SiteArchive {
    pub site_code: String,
    pub active_era: Era,
    pub layers: BTreeMap<Era, ExcavationLayer>,
    pub catalogue: BTreeMap<u8, FindRecord>,
    pub events: Vec<FieldEvent>,
    pub opened_at_ms: u64,
    pub completed_at_ms: Option<u64>,
}

impl SiteArchive {
    pub fn new(site_code: impl Into<String>, timestamp_ms: u64) -> Self {
        let active_era = Era::OptimizedPresent;
        let mut layers = BTreeMap::new();
        layers.insert(active_era, ExcavationLayer::new(active_era, timestamp_ms));
        Self {
            site_code: site_code.into(),
            active_era,
            layers,
            catalogue: BTreeMap::new(),
            events: vec![FieldEvent::LayerOpened { era: active_era }],
            opened_at_ms: timestamp_ms,
            completed_at_ms: None,
        }
    }

    pub fn active_layer(&self) -> &ExcavationLayer {
        self.layers
            .get(&self.active_era)
            .expect("active era must always have an excavation layer")
    }

    pub fn active_layer_mut(&mut self) -> &mut ExcavationLayer {
        self.layers
            .get_mut(&self.active_era)
            .expect("active era must always have an excavation layer")
    }

    pub fn brush(&mut self, sample: BrushSample) -> Vec<FieldEvent> {
        let before = self.active_layer().exposure();
        let timestamp_ms = sample.timestamp_ms;
        let disturbed = self.active_layer_mut().apply_sample(sample);
        let finds = self.active_layer_mut().recover_eligible(timestamp_ms);
        let after = self.active_layer().exposure();
        let era = self.active_era;
        let mut emitted = Vec::new();

        if disturbed > 0 {
            emitted.push(FieldEvent::SurfaceDisturbed { cells: disturbed });
        }
        for record in finds {
            let catalogue_number = record.catalogue_number;
            self.catalogue.insert(catalogue_number, record);
            emitted.push(FieldEvent::ArtifactRecovered { catalogue_number });
        }
        if before < DESCENT_THRESHOLD && after >= DESCENT_THRESHOLD {
            emitted.push(FieldEvent::ThresholdReached { era, exposure: after });
        }
        self.events.extend(emitted.iter().cloned());
        emitted
    }

    pub fn sweep(&mut self, seed: u64, timestamp_ms: u64) -> Vec<FieldEvent> {
        let before = self.active_layer().exposure();
        let era = self.active_era;
        let disturbed = self
            .active_layer_mut()
            .survey_sweep(seed, timestamp_ms);
        let finds = self.active_layer_mut().recover_eligible(timestamp_ms);
        let after = self.active_layer().exposure();
        let mut emitted = vec![FieldEvent::SurfaceDisturbed { cells: disturbed }];
        for record in finds {
            let catalogue_number = record.catalogue_number;
            self.catalogue.insert(catalogue_number, record);
            emitted.push(FieldEvent::ArtifactRecovered { catalogue_number });
        }
        if before < DESCENT_THRESHOLD && after >= DESCENT_THRESHOLD {
            emitted.push(FieldEvent::ThresholdReached { era, exposure: after });
        }
        self.events.extend(emitted.iter().cloned());
        emitted
    }

    pub fn may_descend(&self) -> bool {
        self.active_layer().exposure() >= DESCENT_THRESHOLD
    }

    pub fn descend(&mut self, timestamp_ms: u64) -> Result<Vec<FieldEvent>, ExcavationError> {
        if !self.may_descend() {
            return Err(ExcavationError::InsufficientExposure {
                current: self.active_layer().exposure(),
                required: DESCENT_THRESHOLD,
            });
        }
        let report = self.active_layer().report();
        self.active_layer_mut().completed_at_ms = Some(timestamp_ms);
        let mut emitted = vec![FieldEvent::LayerClosed { report }];
        match self.active_era.next() {
            Some(next) => {
                self.active_era = next;
                self.layers
                    .entry(next)
                    .or_insert_with(|| ExcavationLayer::new(next, timestamp_ms));
                emitted.push(FieldEvent::LayerOpened { era: next });
            }
            None => {
                self.completed_at_ms = Some(timestamp_ms);
                emitted.push(FieldEvent::BedrockReached);
            }
        }
        self.events.extend(emitted.iter().cloned());
        Ok(emitted)
    }

    pub fn reset_active_layer(&mut self, timestamp_ms: u64) {
        let removed: Vec<u8> = self.active_layer().finds.iter().copied().collect();
        for number in removed {
            self.catalogue.remove(&number);
        }
        self.active_layer_mut().reset(timestamp_ms);
        self.events.push(FieldEvent::SiteReset);
    }

    pub fn total_exposure(&self) -> f32 {
        Era::ALL
            .iter()
            .map(|era| self.layers.get(era).map_or(0.0, ExcavationLayer::exposure))
            .sum::<f32>()
            / Era::ALL.len() as f32
    }

    pub fn completion_ratio(&self) -> f32 {
        let layer_component = self
            .layers
            .values()
            .filter(|layer| layer.exposure() >= DESCENT_THRESHOLD)
            .count() as f32
            / Era::ALL.len() as f32;
        let catalogue_component = self.catalogue.len() as f32 / CATALOGUE_SIZE as f32;
        layer_component * 0.65 + catalogue_component * 0.35
    }

    pub fn archive_score(&self) -> u16 {
        let conservation: f32 = self
            .layers
            .values()
            .map(|layer| layer.conservation_score() as f32)
            .sum::<f32>()
            / self.layers.len().max(1) as f32;
        let finds = self.catalogue.len() as f32 / CATALOGUE_SIZE as f32 * 400.0;
        let depth = self.completion_ratio() * 350.0;
        (conservation * 2.5 + finds + depth).clamp(0.0, 1000.0) as u16
    }

    pub fn summary(&self) -> ArchiveSummary {
        ArchiveSummary {
            site_code: self.site_code.clone(),
            active_era: self.active_era,
            layers_opened: self.layers.len(),
            artifacts_recovered: self.catalogue.len(),
            total_exposure: self.total_exposure(),
            completion_ratio: self.completion_ratio(),
            archive_score: self.archive_score(),
            complete: self.completed_at_ms.is_some(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArchiveSummary {
    pub site_code: String,
    pub active_era: Era,
    pub layers_opened: usize,
    pub artifacts_recovered: usize,
    pub total_exposure: f32,
    pub completion_ratio: f32,
    pub archive_score: u16,
    pub complete: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ExcavationError {
    InsufficientExposure { current: f32, required: f32 },
    InvalidSample,
    Serialization(String),
}

impl std::fmt::Display for ExcavationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExcavationError::InsufficientExposure { current, required } => write!(
                formatter,
                "layer exposure {:.1}% is below the required {:.1}%",
                current * 100.0,
                required * 100.0
            ),
            ExcavationError::InvalidSample => write!(formatter, "brush sample is not finite"),
            ExcavationError::Serialization(message) => write!(formatter, "{message}"),
        }
    }
}

fn pseudo_random(seed: u64) -> f32 {
    let mut value = seed.wrapping_add(0x9e3779b97f4a7c15);
    value = (value ^ (value >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94d049bb133111eb);
    value ^= value >> 31;
    (value as f64 / u64::MAX as f64) as f32
}

#[wasm_bindgen]
pub struct WasmExcavation {
    archive: SiteArchive,
}

#[wasm_bindgen]
impl WasmExcavation {
    #[wasm_bindgen(constructor)]
    pub fn new(timestamp_ms: f64) -> WasmExcavation {
        WasmExcavation {
            archive: SiteArchive::new("WA-09", timestamp_ms.max(0.0) as u64),
        }
    }

    pub fn brush(
        &mut self,
        x: f32,
        y: f32,
        radius: f32,
        pressure: f32,
        timestamp_ms: f64,
    ) -> Result<String, JsValue> {
        if !x.is_finite() || !y.is_finite() || !radius.is_finite() || !pressure.is_finite() {
            return Err(JsValue::from_str(&ExcavationError::InvalidSample.to_string()));
        }
        let events = self.archive.brush(BrushSample {
            x,
            y,
            radius,
            pressure,
            timestamp_ms: timestamp_ms.max(0.0) as u64,
        });
        serde_json::to_string(&events).map_err(|error| JsValue::from_str(&error.to_string()))
    }

    pub fn survey_sweep(&mut self, seed: f64, timestamp_ms: f64) -> Result<String, JsValue> {
        let events = self
            .archive
            .sweep(seed.max(0.0) as u64, timestamp_ms.max(0.0) as u64);
        serde_json::to_string(&events).map_err(|error| JsValue::from_str(&error.to_string()))
    }

    pub fn descend(&mut self, timestamp_ms: f64) -> Result<String, JsValue> {
        let events = self
            .archive
            .descend(timestamp_ms.max(0.0) as u64)
            .map_err(|error| JsValue::from_str(&error.to_string()))?;
        serde_json::to_string(&events).map_err(|error| JsValue::from_str(&error.to_string()))
    }

    pub fn reset_layer(&mut self, timestamp_ms: f64) {
        self.archive.reset_active_layer(timestamp_ms.max(0.0) as u64);
    }

    pub fn report(&self) -> Result<String, JsValue> {
        serde_json::to_string(&self.archive.active_layer().report())
            .map_err(|error| JsValue::from_str(&error.to_string()))
    }

    pub fn summary(&self) -> Result<String, JsValue> {
        serde_json::to_string(&self.archive.summary())
            .map_err(|error| JsValue::from_str(&error.to_string()))
    }

    pub fn catalogue(&self) -> Result<String, JsValue> {
        let entries: Vec<(&Artifact, Option<&FindRecord>)> = ARTIFACTS
            .iter()
            .map(|artifact| {
                (
                    artifact,
                    self.archive.catalogue.get(&artifact.catalogue_number),
                )
            })
            .collect();
        serde_json::to_string(&entries).map_err(|error| JsValue::from_str(&error.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample(x: f32, y: f32, time: u64) -> BrushSample {
        BrushSample {
            x,
            y,
            radius: 0.12,
            pressure: 0.8,
            timestamp_ms: time,
        }
    }

    #[test]
    fn all_eras_are_chronological() {
        let years: Vec<u16> = Era::ALL.iter().map(|era| era.year()).collect();
        assert_eq!(years, vec![2026, 2012, 2003, 1996]);
    }

    #[test]
    fn brush_samples_disturb_the_grid() {
        let mut layer = ExcavationLayer::new(Era::OptimizedPresent, 0);
        assert_eq!(layer.disturbed_cells(), 0);
        layer.apply_sample(sample(0.5, 0.5, 10));
        assert!(layer.disturbed_cells() > 0);
        assert!(layer.exposure() > 0.0);
    }

    #[test]
    fn repeated_excavation_recovers_artifacts() {
        let mut archive = SiteArchive::new("TEST", 0);
        for pass in 0..180 {
            let x = ((pass * 7) % GRID_COLUMNS) as f32 / GRID_COLUMNS as f32;
            let y = ((pass * 11) % GRID_ROWS) as f32 / GRID_ROWS as f32;
            archive.brush(sample(x, y, pass as u64 * 16));
        }
        assert!(!archive.catalogue.is_empty());
    }

    #[test]
    fn premature_descent_is_rejected() {
        let mut archive = SiteArchive::new("TEST", 0);
        let result = archive.descend(100);
        assert!(matches!(
            result,
            Err(ExcavationError::InsufficientExposure { .. })
        ));
    }

    #[test]
    fn survey_sweep_is_deterministic() {
        let mut first = ExcavationLayer::new(Era::ResponsiveTurn, 0);
        let mut second = ExcavationLayer::new(Era::ResponsiveTurn, 0);
        first.survey_sweep(42, 100);
        second.survey_sweep(42, 100);
        assert_eq!(first.exposure(), second.exposure());
        assert_eq!(first.disturbed_cells(), second.disturbed_cells());
    }

    #[test]
    fn catalogue_numbers_are_unique() {
        let numbers: BTreeSet<u8> = ARTIFACTS
            .iter()
            .map(|artifact| artifact.catalogue_number)
            .collect();
        assert_eq!(numbers.len(), CATALOGUE_SIZE);
    }

    #[test]
    fn each_era_has_three_artifacts() {
        for era in Era::ALL {
            assert_eq!(
                ARTIFACTS.iter().filter(|artifact| artifact.era == era).count(),
                3
            );
        }
    }

    #[test]
    fn reset_removes_active_layer_finds() {
        let mut archive = SiteArchive::new("TEST", 0);
        archive.active_layer_mut().finds.insert(1);
        archive.catalogue.insert(
            1,
            FindRecord {
                catalogue_number: 1,
                era: Era::OptimizedPresent,
                discovered_at_ms: 10,
                exposure_at_discovery: 0.2,
                context_preserved: true,
                field_coordinates: (9, 8),
            },
        );
        archive.reset_active_layer(20);
        assert!(archive.catalogue.is_empty());
        assert!(archive.active_layer().finds.is_empty());
    }

    #[test]
    fn summary_begins_in_the_present() {
        let archive = SiteArchive::new("WA-09", 0);
        let summary = archive.summary();
        assert_eq!(summary.active_era, Era::OptimizedPresent);
        assert_eq!(summary.layers_opened, 1);
        assert_eq!(summary.artifacts_recovered, 0);
        assert!(!summary.complete);
    }
}
