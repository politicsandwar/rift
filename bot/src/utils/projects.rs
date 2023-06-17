pub fn get_project(bits: i64, project: ProjectStrs) -> bool {
    bits & (1 << project as u8) != 0
}

#[allow(dead_code)]
pub enum ProjectStrs {
    Ironworks,
    Bauxiteworks,
    ArmsStockpile,
    EmergencyGasolineReserve,
    MassIrrigation,
    InternationalTradeCenter,
    MissileLaunchPad,
    NuclearResearchFacility,
    IronDome,
    VitalDefenseSystem,
    CentralIntelligenceAgency,
    CenterforCivilEngineering,
    PropagandaBureau,
    UraniumEnrichmentProgram,
    UrbanPlanning,
    AdvancedUrbanPlanning,
    SpaceProgram,
    SpySatellite,
    MoonLanding,
    PirateEconomy,
    RecyclingInitiative,
    TelecommunicationsSatellite,
    GreenTechnologies,
    ArableLandAgency,
    ClinicalResearchCenter,
    SpecializedPoliceTrainingProgram,
    AdvancedEngineeringCorps,
    GovernmentSupportAgency,
    ResearchandDevelopmentCenter,
    ResourceProductionCenter,
    MetropolitanPlanning,
    MilitarySalvage,
    FalloutShelter,
}
