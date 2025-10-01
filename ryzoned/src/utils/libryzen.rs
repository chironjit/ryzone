use libryzenadj::RyzenAdj;

pub enum RyzenSetter {
    SetFastLimit(u32),      // PPT Fast limit in mW
    SetSlowLimit(u32),      // PPT Slow limit in mW
    SetStapmLimit(u32),     // STAPM limit in mW
    SetTctlTemp(f64),       // Temperature limit in °C
}

pub enum RyzenGetter {
    GetFastLimit,           // Get PPT Fast limit
    GetFastValue,           // Get current PPT Fast value
    GetSlowLimit,           // Get PPT Slow limit
    GetSlowValue,           // Get current PPT Slow value
    GetStapmLimit,          // Get STAPM limit
    GetStapmValue,          // Get current STAPM value
    GetTctlTemp,            // Get temperature limit
    GetTctlTempValue,       // Get current temperature value
}

impl RyzenSetter {
    pub fn execute(&self, ryzen: &RyzenAdj) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            RyzenSetter::SetFastLimit(value) => ryzen.set_fast_limit(*value),
            RyzenSetter::SetSlowLimit(value) => ryzen.set_slow_limit(*value),
            RyzenSetter::SetStapmLimit(value) => ryzen.set_stapm_limit(*value),
            RyzenSetter::SetTctlTemp(value) => ryzen.set_tctl_temp(*value),
        }
    }
}

impl RyzenGetter {
    pub fn execute(&self, ryzen: &RyzenAdj) -> Result<f64, Box<dyn std::error::Error>> {
        match self {
            RyzenGetter::GetFastLimit => Ok(ryzen.get_fast_limit().unwrap_or_default()),
            RyzenGetter::GetFastValue => Ok(ryzen.get_fast_value().unwrap_or_default()),
            RyzenGetter::GetSlowLimit => Ok(ryzen.get_slow_limit().unwrap_or_default()),
            RyzenGetter::GetSlowValue => Ok(ryzen.get_slow_value().unwrap_or_default()),
            RyzenGetter::GetStapmLimit => Ok(ryzen.get_stapm_limit().unwrap_or_default()),
            RyzenGetter::GetStapmValue => Ok(ryzen.get_stapm_value().unwrap_or_default()),
            RyzenGetter::GetTctlTemp => Ok(ryzen.get_tctl_temp().unwrap_or_default()),
            RyzenGetter::GetTctlTempValue => Ok(ryzen.get_tctl_temp_value().unwrap_or_default()),
        }
    }
}