//! Core BIM data model for ConstructCraft.
//!
//! The [`Project`] is the single source of truth. Every view, schedule and
//! sheet is a *query* over it; all geometry is derived. Edits go through
//! [`Document::edit`], which records a [`Transaction`] for undo/redo.

pub mod arc;
pub mod cad;
pub mod callout;
pub mod component;
pub mod component_file;
pub mod component_ref;
pub mod component_script;
pub mod curtain;
pub mod design_option;
pub mod detail;
pub mod detail_component;
pub mod dim_line;
pub mod dim_refs;
mod document;
mod element;
pub mod expr;
pub mod fill_pattern;
pub mod graph_binding;
pub mod graphics;
pub mod grid;
mod ids;
pub mod level_mark;
pub mod lightning;
pub mod mass;
pub mod material;
pub mod mep;
pub mod openings;
pub mod pad;
pub mod paint;
pub mod pat;
mod persist;
pub mod phase;
pub mod profile;
pub mod profile_sweep;
mod project;
pub mod properties;
pub mod ramp;
pub mod raster;
pub mod recovery;
pub mod region;
mod repair;
pub mod roofing;
mod schedule;
pub mod shaft;
pub mod site;
pub mod sketch;
pub mod sketch_edit;
pub mod stair;
pub mod sweep;
pub mod title_block;
pub mod topography;
pub mod transform;
mod types;
pub mod units;
mod view;
pub mod wall_ops;

pub use cad::{CadColorMode, CadDrawing, CadImport, CadLayer, CadPrim, CadUnit};
pub use callout::{Callout, CropBox};
pub use component::{
    ComponentDef, ComponentItem, ComponentKind, ComponentParam, ComponentShape, DoorOperation, ParamKind, ResolveCtx,
    SymbolLine, WindowOperation,
};
pub use component_ref::{PlaneAxis, RefDimension, RefEqual, RefLine, RefPlane};
pub use component_script::{Hotspot, HotspotShape, ParamChoice, ScriptEval, ScriptInput, ScriptOwner, ScriptResult};
pub use design_option::{DesignOption, OptionSet};
pub use detail::{DetailLine, DetailStyle, LineStyleDef, LineStyleRef};
pub use detail_component::DetailComponent;
pub use dim_refs::{DimRef, Projection, RefAnchor, RefHeight, RefTarget};
pub use document::{Document, EditCtx, Transaction};
pub use element::{
    BarFace, Beam, BeamLateral, BeamVertical, Ceiling, Column, Component, Dimension, DimensionKind, Element,
    ElementKind, Floor, Foundation, FoundationShape, Grid, Hosted, Leader, LeaderSymbol, LocationLine, Railing, Ramp,
    Rebar, RebarLayout, Roof, RoofShape, Room, Stair, TextAlignment, TextNote, Wall, WallTop,
};
pub use fill_pattern::{FillPattern, FillPatternKind, PatternLine};
pub use graphics::{
    CategoryGraphics, FillGraphics, FilterOp, FilterParam, FilterRule, FilterUse, GraphicOverride, LineGraphics,
    LinePattern, TemplateIncludes, TemporaryVisibility, ViewFilter, ViewGraphics, ViewTemplate,
};
pub use ids::{
    ElementId, ElementTypeId, FillPatternId, FilterId, GroupId, LevelId, LineStyleId, OptionId, OptionSetId,
    PhaseFilterId, PhaseId, SystemId, TemplateId, Uid, ViewId,
};
pub use lightning::{ConductorRole, LightningConductor, LpsClass};
pub use material::{Material, MaterialClass, Pattern, PatternFill};
pub use mep::{DeviceKind, MepData, MepSegment, MepSystem, SystemClass, SystemKind, TrayStyle, Valve, ValveKind};
pub use persist::{
    FORMAT_VERSION, PersistError, from_ron, from_ron_with_repairs, from_ron_with_repairs_localized, load,
    load_with_repairs, load_with_repairs_localized, save, to_ron,
};
pub use phase::{Phase, PhaseDisplay, PhaseFilter, PhaseGraphics, PhaseState, PhaseStatus};
pub use profile::{ProfilePoint, SketchPlane};
pub use project::{
    Group, Level, Project, ProjectField, ProjectInfo, can_join_geometry, join_priority, template_default,
};
pub use ramp::{RampCheck, RampIssue, RampLayout, RampRules, RampShape, ramp_flights};
pub use region::Region;
pub use schedule::{ScheduleDef, ScheduleField};
pub use stair::{
    LandingBeams, LandingPosts, PostAt, StairBuild, StairCheck, StairConstruction, StairIssue, StairLayout,
    StairRailing, StairRules, StairShape, StairSide, Stringers, TreadRule, Turn, stair_steps,
};
pub use transform::PlanTransform;
pub use types::{
    Category, ColumnSection, ElementType, FoundationKind, Layer, LayerFunction, SectionShape, TypeData, TypeInfo,
    WallFunction, stair_layout,
};
pub use units::{AreaUnit, LengthUnit, NumberStyle, SlopeUnit, Units, VolumeUnit};
pub use view::{
    CameraState, DetailLevel, Direction, Looking, MAX_PAPER_MM, MAX_SECTION_SEGMENT, MIN_PAPER_MM, MepDiscipline,
    SheetSize, Underlay, UnderlayLevel, UnderlayPlan, View, ViewCategory, ViewKind, Viewport, VisualStyle,
};

pub use glam::{DVec2, DVec3};
