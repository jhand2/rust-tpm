use crate::tpm::*;
use crate::types::*;

fn get_tpm_property(
    _tpm: &mut TpmInstance,
    property: TpmPt,
    _count: u32,
) -> Result<TpmuCapabilityData, TpmError> {
    let mut props =
        TpmuCapabilityData::TpmProperties(0, [TpmsTaggedProperty::default(); MAX_TPM_PROPERTIES]);

    match property {
        // TODO: Put a real manufacturer ID
        TpmPt::Manufacturer => {
            if let TpmuCapabilityData::TpmProperties(ref mut count, ref mut properties) = props {
                *count = 1;
                properties[0].property = property;
                properties[0].val = 0x0;
            }
        }
        _ => return Err(TpmError { rc: TpmRc::Value }),
    };

    Ok(props)
}

pub fn tpm2_get_capability(
    tpm: &mut TpmInstance,
    args: &GetCapabilityArgs,
) -> Result<GetCapabilityResponse, TpmError> {
    let mut response = GetCapabilityResponse::default();

    response.data = match args.cap {
        TpmCapability::TpmProperty => {
            match get_tpm_property(tpm, args.property, args.property_count) {
                Ok(data) => data,
                Err(e) => return Err(e),
            }
        }
        _ => return Err(TpmError { rc: TpmRc::Value }),
    };

    Ok(response)
}
