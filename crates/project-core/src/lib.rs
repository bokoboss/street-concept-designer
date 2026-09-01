//! Project and scenario domain semantics above the accepted R1 engineering
//! kernel.
//!
//! This crate owns durable project identity, scenario ownership, and the
//! minimum project context needed before persistence and command history. It
//! intentionally contains no renderer, file-format, UI, or command-engine
//! state. A scenario exposes its network only through an immutable reference;
//! network replacement is a controlled, lock-aware operation.

use std::fmt::{Display, Formatter};

pub use street_concept_designer_kernel::SemanticRef;
use street_concept_designer_kernel::{KernelError, RoadNetwork, TolerancePolicy};

/// Stable semantic identity for one project.
///
/// The value is supplied by the caller. It is not derived from the project
/// name, a file path, a timestamp, or a random source.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProjectId(String);

impl ProjectId {
    /// Construct a non-empty, whitespace-free project identity.
    pub fn new(value: impl Into<String>) -> Result<Self, ProjectError> {
        let value = value.into();
        if value.is_empty() || value.chars().any(char::is_whitespace) {
            return Err(ProjectError::InvalidProjectId);
        }
        Ok(Self(value))
    }

    /// Borrow the stable identity text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for ProjectId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// Stable semantic identity for one scenario within a project.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ScenarioId(String);

impl ScenarioId {
    /// Construct a non-empty, whitespace-free scenario identity.
    pub fn new(value: impl Into<String>) -> Result<Self, ProjectError> {
        let value = value.into();
        if value.is_empty() || value.chars().any(char::is_whitespace) {
            return Err(ProjectError::InvalidScenarioId);
        }
        Ok(Self(value))
    }

    /// Borrow the stable identity text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for ScenarioId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// Initial semantic role of a scenario.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScenarioRole {
    /// Current or reference design state.
    Existing,
    /// A proposed design variant.
    Alternative,
}

/// Alias for callers that use the broader "kind" terminology.
pub type ScenarioKind = ScenarioRole;

/// Explicit traffic-side context for the engineering model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TrafficSide {
    /// Left-hand traffic.
    LeftHand,
    /// Right-hand traffic.
    RightHand,
}

impl TrafficSide {
    /// Whether this context uses left-hand traffic.
    pub fn is_left_hand(self) -> bool {
        matches!(self, Self::LeftHand)
    }

    /// Whether this context uses right-hand traffic.
    pub fn is_right_hand(self) -> bool {
        matches!(self, Self::RightHand)
    }
}

/// Minimal durable coordinate interpretation metadata.
///
/// The reference value may identify a CRS or another caller-defined
/// coordinate reference. This type records context only; it does not perform
/// coordinate transformations or represent the R1C render-local origin.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CoordinateContext {
    reference_id: Option<String>,
    description: Option<String>,
}

impl CoordinateContext {
    /// Construct an empty, explicitly local coordinate context.
    pub fn new() -> Self {
        Self::default()
    }

    /// Alias for [`Self::new`].
    pub fn empty() -> Self {
        Self::new()
    }

    /// Construct context from optional reference and descriptive metadata.
    pub fn from_parts(
        reference_id: Option<String>,
        description: Option<String>,
    ) -> Result<Self, ProjectError> {
        let context = Self {
            reference_id,
            description,
        };
        context.validate()?;
        Ok(context)
    }

    /// Set a non-empty reference/CRS identifier.
    pub fn with_reference_id(
        mut self,
        reference_id: impl Into<String>,
    ) -> Result<Self, ProjectError> {
        let reference_id = reference_id.into();
        if reference_id.is_empty() || reference_id.chars().any(char::is_whitespace) {
            return Err(ProjectError::InvalidCoordinateContext);
        }
        self.reference_id = Some(reference_id);
        Ok(self)
    }

    /// Alias emphasizing CRS use of the reference identifier.
    pub fn with_crs_identifier(
        self,
        crs_identifier: impl Into<String>,
    ) -> Result<Self, ProjectError> {
        self.with_reference_id(crs_identifier)
    }

    /// Set descriptive coordinate-context metadata.
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> Result<Self, ProjectError> {
        let description = description.into();
        if description.trim().is_empty() {
            return Err(ProjectError::InvalidCoordinateContext);
        }
        self.description = Some(description);
        Ok(self)
    }

    /// Optional CRS/reference identifier.
    pub fn reference_id(&self) -> Option<&str> {
        self.reference_id.as_deref()
    }

    /// Alias for [`Self::reference_id`].
    pub fn crs_identifier(&self) -> Option<&str> {
        self.reference_id()
    }

    /// Optional descriptive metadata.
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    fn validate(&self) -> Result<(), ProjectError> {
        if self
            .reference_id
            .as_deref()
            .is_some_and(|value| value.is_empty() || value.chars().any(char::is_whitespace))
            || self
                .description
                .as_deref()
                .is_some_and(|value| value.trim().is_empty())
        {
            return Err(ProjectError::InvalidCoordinateContext);
        }
        Ok(())
    }
}

/// Project-domain failures that are distinct from low-level kernel failures.
#[derive(Debug, Clone, PartialEq)]
pub enum ProjectError {
    /// The caller supplied an empty or whitespace-containing project id.
    InvalidProjectId,
    /// The caller supplied an empty or whitespace-containing scenario id.
    InvalidScenarioId,
    /// A project cannot contain two scenarios with one identity.
    DuplicateScenarioId,
    /// The requested scenario does not exist in the project.
    MissingScenario,
    /// The project must contain at least one scenario after construction.
    NoScenarios,
    /// A locked scenario cannot receive ordinary semantic mutations.
    ScenarioLocked,
    /// Removing the last scenario would violate the project root invariant.
    LastScenarioRemoval,
    /// Coordinate-reference metadata is empty or otherwise malformed.
    InvalidCoordinateContext,
    /// The accepted kernel rejected the supplied network or validation input.
    Kernel(KernelError),
}

impl Display for ProjectError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidProjectId => write!(f, "invalid project id"),
            Self::InvalidScenarioId => write!(f, "invalid scenario id"),
            Self::DuplicateScenarioId => write!(f, "duplicate scenario id"),
            Self::MissingScenario => write!(f, "missing scenario"),
            Self::NoScenarios => write!(f, "project must contain at least one scenario"),
            Self::ScenarioLocked => write!(f, "scenario is locked"),
            Self::LastScenarioRemoval => write!(f, "cannot remove the last scenario"),
            Self::InvalidCoordinateContext => write!(f, "invalid coordinate context"),
            Self::Kernel(error) => Display::fmt(error, f),
        }
    }
}

impl std::error::Error for ProjectError {}

impl From<KernelError> for ProjectError {
    fn from(error: KernelError) -> Self {
        Self::Kernel(error)
    }
}

/// One scenario-owned engineering network and its project metadata.
#[derive(Debug, Clone, PartialEq)]
pub struct Scenario {
    id: ScenarioId,
    name: String,
    role: ScenarioRole,
    locked: bool,
    network: RoadNetwork,
}

impl Scenario {
    /// Construct a scenario after validating its network with the R1 default
    /// numerical policy.
    pub fn new(
        id: ScenarioId,
        name: impl Into<String>,
        role: ScenarioRole,
        locked: bool,
        network: RoadNetwork,
    ) -> Result<Self, ProjectError> {
        Self::new_with_policy(id, name, role, locked, network, &TolerancePolicy::default())
    }

    /// Construct a scenario using an explicitly selected kernel policy.
    pub fn new_with_policy(
        id: ScenarioId,
        name: impl Into<String>,
        role: ScenarioRole,
        locked: bool,
        network: RoadNetwork,
        policy: &TolerancePolicy,
    ) -> Result<Self, ProjectError> {
        network.validate(policy)?;
        Ok(Self {
            id,
            name: name.into(),
            role,
            locked,
            network,
        })
    }

    /// Stable identity within its owning project.
    pub fn id(&self) -> &ScenarioId {
        &self.id
    }

    /// Editable display name; it is not identity.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Semantic scenario role.
    pub fn role(&self) -> ScenarioRole {
        self.role
    }

    /// Canonical protection intent.
    pub fn is_locked(&self) -> bool {
        self.locked
    }

    /// Immutable scenario-owned engineering network.
    pub fn network(&self) -> &RoadNetwork {
        &self.network
    }

    /// Revalidate this scenario and its network.
    pub fn validate(&self, policy: &TolerancePolicy) -> Result<(), ProjectError> {
        if ScenarioId::new(self.id.as_str()).is_err() {
            return Err(ProjectError::InvalidScenarioId);
        }
        self.network.validate(policy)?;
        Ok(())
    }

    /// Rename an editable scenario. Locked scenarios require an explicit
    /// unlock operation before ordinary metadata mutation.
    pub fn rename(&mut self, name: impl Into<String>) -> Result<(), ProjectError> {
        self.ensure_editable()?;
        self.name = name.into();
        Ok(())
    }

    /// Set the canonical lock flag. Lock control itself is explicit and is
    /// therefore allowed for both locking and unlocking.
    pub fn set_locked(&mut self, locked: bool) {
        self.locked = locked;
    }

    /// Replace the complete network through a lock-aware validated operation.
    /// No mutable network reference is exposed.
    pub fn replace_network(
        &mut self,
        network: RoadNetwork,
        policy: &TolerancePolicy,
    ) -> Result<(), ProjectError> {
        self.ensure_editable()?;
        network.validate(policy)?;
        self.network = network;
        Ok(())
    }

    fn duplicate(
        &self,
        new_id: ScenarioId,
        name: impl Into<String>,
        role: ScenarioRole,
        locked: bool,
    ) -> Self {
        Self {
            id: new_id,
            name: name.into(),
            role,
            locked,
            network: self.network.clone(),
        }
    }

    fn ensure_editable(&self) -> Result<(), ProjectError> {
        if self.locked {
            Err(ProjectError::ScenarioLocked)
        } else {
            Ok(())
        }
    }
}

/// Project-level identity that adds scenario scope to an R1 scenario-local
/// [`SemanticRef`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProjectSemanticRef {
    scenario_id: ScenarioId,
    semantic_ref: SemanticRef,
}

impl ProjectSemanticRef {
    /// Construct a project-scoped reference without changing the kernel's
    /// scenario-local [`SemanticRef`] type.
    pub fn new(scenario_id: ScenarioId, semantic_ref: SemanticRef) -> Self {
        Self {
            scenario_id,
            semantic_ref,
        }
    }

    /// Scenario scope of the reference.
    pub fn scenario_id(&self) -> &ScenarioId {
        &self.scenario_id
    }

    /// Scenario-local semantic identity.
    pub fn semantic_ref(&self) -> &SemanticRef {
        &self.semantic_ref
    }
}

/// Canonical project root for R2A.
#[derive(Debug, Clone, PartialEq)]
pub struct Project {
    id: ProjectId,
    name: String,
    traffic_side: TrafficSide,
    coordinate_context: CoordinateContext,
    scenarios: Vec<Scenario>,
}

impl Project {
    /// Construct an empty project shell.
    ///
    /// The shell is intentionally allowed as an intermediate builder state;
    /// [`Self::validate`] rejects it until at least one scenario is added.
    pub fn new(
        id: ProjectId,
        name: impl Into<String>,
        traffic_side: TrafficSide,
        coordinate_context: CoordinateContext,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            traffic_side,
            coordinate_context,
            scenarios: Vec::new(),
        }
    }

    /// Construct and validate a project containing one scenario.
    pub fn with_scenario(
        id: ProjectId,
        name: impl Into<String>,
        traffic_side: TrafficSide,
        coordinate_context: CoordinateContext,
        scenario: Scenario,
    ) -> Result<Self, ProjectError> {
        let mut project = Self::new(id, name, traffic_side, coordinate_context);
        project.add_scenario(scenario)?;
        Ok(project)
    }

    /// Construct and validate a project from an authored scenario list,
    /// retaining the caller-provided scenario order.
    pub fn from_scenarios(
        id: ProjectId,
        name: impl Into<String>,
        traffic_side: TrafficSide,
        coordinate_context: CoordinateContext,
        scenarios: Vec<Scenario>,
    ) -> Result<Self, ProjectError> {
        let mut project = Self::new(id, name, traffic_side, coordinate_context);
        for scenario in scenarios {
            project.add_scenario(scenario)?;
        }
        project.validate(&TolerancePolicy::default())?;
        Ok(project)
    }

    /// Stable project identity.
    pub fn id(&self) -> &ProjectId {
        &self.id
    }

    /// Project display name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Explicit traffic-side context.
    pub fn traffic_side(&self) -> TrafficSide {
        self.traffic_side
    }

    /// Minimal coordinate interpretation metadata.
    pub fn coordinate_context(&self) -> &CoordinateContext {
        &self.coordinate_context
    }

    /// Scenarios in canonical authored order.
    pub fn scenarios(&self) -> &[Scenario] {
        &self.scenarios
    }

    /// Find a scenario by stable identity.
    pub fn scenario(&self, id: &ScenarioId) -> Option<&Scenario> {
        self.scenarios.iter().find(|scenario| scenario.id() == id)
    }

    /// Rename the project without changing its identity.
    pub fn rename(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    /// Change explicit traffic-side context.
    pub fn set_traffic_side(&mut self, traffic_side: TrafficSide) {
        self.traffic_side = traffic_side;
    }

    /// Replace coordinate metadata after validating it.
    pub fn set_coordinate_context(
        &mut self,
        coordinate_context: CoordinateContext,
    ) -> Result<(), ProjectError> {
        coordinate_context.validate()?;
        self.coordinate_context = coordinate_context;
        Ok(())
    }

    /// Append a unique scenario to the current authored order.
    pub fn add_scenario(&mut self, scenario: Scenario) -> Result<(), ProjectError> {
        if self
            .scenarios
            .iter()
            .any(|existing| existing.id() == scenario.id())
        {
            return Err(ProjectError::DuplicateScenarioId);
        }
        scenario.validate(&TolerancePolicy::default())?;
        self.scenarios.push(scenario);
        Ok(())
    }

    /// Remove an editable scenario while preserving the at-least-one
    /// scenario project invariant.
    pub fn remove_scenario(&mut self, id: &ScenarioId) -> Result<Scenario, ProjectError> {
        let index = self
            .scenarios
            .iter()
            .position(|scenario| scenario.id() == id)
            .ok_or(ProjectError::MissingScenario)?;
        if self.scenarios.len() <= 1 {
            return Err(ProjectError::LastScenarioRemoval);
        }
        if self.scenarios[index].is_locked() {
            return Err(ProjectError::ScenarioLocked);
        }
        Ok(self.scenarios.remove(index))
    }

    /// Rename an editable scenario.
    pub fn rename_scenario(
        &mut self,
        id: &ScenarioId,
        name: impl Into<String>,
    ) -> Result<(), ProjectError> {
        self.scenario_mut_for_edit(id)?.rename(name)
    }

    /// Set a scenario's canonical lock state. Lock control is explicit.
    pub fn set_scenario_locked(
        &mut self,
        id: &ScenarioId,
        locked: bool,
    ) -> Result<(), ProjectError> {
        self.scenario_mut(id)?.set_locked(locked);
        Ok(())
    }

    /// Replace a scenario network through the lock-aware domain boundary.
    pub fn replace_scenario_network(
        &mut self,
        id: &ScenarioId,
        network: RoadNetwork,
        policy: &TolerancePolicy,
    ) -> Result<(), ProjectError> {
        self.scenario_mut(id)?.replace_network(network, policy)
    }

    /// Duplicate a scenario with explicit new identity, display metadata,
    /// role, and lock policy. The source's local semantic ids are retained.
    /// The duplicate is inserted immediately after its source in authored
    /// order. Repeating the operation for one source places the newest
    /// duplicate immediately after that source, before earlier duplicates.
    pub fn duplicate_scenario(
        &mut self,
        source_id: &ScenarioId,
        new_id: ScenarioId,
        name: impl Into<String>,
        role: ScenarioRole,
        locked: bool,
    ) -> Result<ScenarioId, ProjectError> {
        if self
            .scenarios
            .iter()
            .any(|scenario| scenario.id() == &new_id)
        {
            return Err(ProjectError::DuplicateScenarioId);
        }
        let source_index = self
            .scenarios
            .iter()
            .position(|scenario| scenario.id() == source_id)
            .ok_or(ProjectError::MissingScenario)?;
        let duplicate = self.scenarios[source_index].duplicate(new_id.clone(), name, role, locked);
        duplicate.validate(&TolerancePolicy::default())?;
        self.scenarios.insert(source_index + 1, duplicate);
        Ok(new_id)
    }

    /// Construct a project-scoped semantic reference after confirming that
    /// its scenario scope exists in this project.
    pub fn semantic_ref(
        &self,
        scenario_id: &ScenarioId,
        semantic_ref: SemanticRef,
    ) -> Result<ProjectSemanticRef, ProjectError> {
        if self.scenario(scenario_id).is_none() {
            return Err(ProjectError::MissingScenario);
        }
        Ok(ProjectSemanticRef::new(scenario_id.clone(), semantic_ref))
    }

    /// Deterministically validate project metadata, scenario identities, and
    /// every owned R1 network.
    pub fn validate(&self, policy: &TolerancePolicy) -> Result<(), ProjectError> {
        if ProjectId::new(self.id.as_str()).is_err() {
            return Err(ProjectError::InvalidProjectId);
        }
        self.coordinate_context.validate()?;
        if self.scenarios.is_empty() {
            return Err(ProjectError::NoScenarios);
        }
        for (index, scenario) in self.scenarios.iter().enumerate() {
            if ScenarioId::new(scenario.id.as_str()).is_err() {
                return Err(ProjectError::InvalidScenarioId);
            }
            if self.scenarios[..index]
                .iter()
                .any(|previous| previous.id() == scenario.id())
            {
                return Err(ProjectError::DuplicateScenarioId);
            }
            scenario.validate(policy)?;
        }
        Ok(())
    }

    /// Validate using the accepted R1 default numerical policy.
    pub fn validate_default(&self) -> Result<(), ProjectError> {
        self.validate(&TolerancePolicy::default())
    }

    fn scenario_mut(&mut self, id: &ScenarioId) -> Result<&mut Scenario, ProjectError> {
        self.scenarios
            .iter_mut()
            .find(|scenario| scenario.id() == id)
            .ok_or(ProjectError::MissingScenario)
    }

    fn scenario_mut_for_edit(&mut self, id: &ScenarioId) -> Result<&mut Scenario, ProjectError> {
        let scenario = self.scenario_mut(id)?;
        if scenario.is_locked() {
            return Err(ProjectError::ScenarioLocked);
        }
        Ok(scenario)
    }
}
