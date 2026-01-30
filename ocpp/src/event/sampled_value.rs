use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum SampledValue {
    CurrentExport {
        ampere: f64,
        context: (),
        phase: (),
        location: (),
        signed_meter_value: (),
    },
    CurrentImport,
    CurrentOffered,
    EnergyActiveExportRegister,
    EnergyActiveImportRegister,
    EnergyReactiveExportRegister,
    EnergyReactiveImportRegister,
    EnergyActiveExportInterval,
    EnergyActiveImportInterval,
    EnergyActiveNet,
    EnergyReactiveExportInterval,
    EnergyReactiveImportInterval,
    EnergyReactiveNet,
    EnergyApparentNet,
    EnergyApparentImport,
    EnergyApparentExport,
    Frequency,
    PowerActiveExport,
    PowerActiveImport,
    PowerFactor,
    PowerOffered,
    PowerReactiveExport,
    PowerReactiveImport,
    SoC,
    Voltage,
}
