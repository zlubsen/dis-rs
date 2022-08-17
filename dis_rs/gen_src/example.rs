/// example:
///
/// ```rust
/// #[derive(PduEnum)]
/// #[uid(7))
/// pub enum EntityKind {
/// }
/// ```
///
/// Should generate the following based on the SISO-REF-010 v-something file (as defined in cargo.toml?):
///
/// ```rust
/// pub enum EntityKind {
///     Other = 0,
///     Platform = 1,
///     Munition = 2,
///     LifeForm = 3,
///     Environmental = 4,
///     CulturalFeature = 5,
///     Supply = 6,
///     Radio = 7,
///     Expendable = 8,
///     SensorEmitter = 9,
/// }
/// ```
/// And the associated From<u8> and From<EntityKind> impls
