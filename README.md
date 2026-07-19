# plugin-api

Canonical league data schema and WIT plugin interface shared by the FullTime plugin host
(`Apps/rust`) and data-provider plugins (`Plugins/Bundesliga`, and future league plugins).

Neither the host nor any plugin owns this contract - it is versioned and published
independently so plugins and the host can evolve without a lockstep release.
