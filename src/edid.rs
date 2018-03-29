#[derive(Debug)]
pub struct Edid {
    edid: Box<[u8]>,
}

const EDID_MIN_LENGTH: usize = 128;
const EDID_BYTE_MAX_CM_HORIZ: usize = 0x15;
const EDID_BYTE_MAX_CM_VERT: usize = 0x16;

impl Edid {
    pub fn new(edid: &[u8], name: &str) -> Edid {
        assert!(
            edid.len() >= EDID_MIN_LENGTH,
            format!(
                "{} has edid size {}, expected at least {}",
                name,
                edid.len(),
                EDID_MIN_LENGTH
            )
        );

        Edid {
            edid: Vec::from(edid).into_boxed_slice(),
        }
    }

    pub fn max_cm_horiz(&self) -> u32 {
        u32::from(self.edid[EDID_BYTE_MAX_CM_HORIZ])
    }

    pub fn max_cm_vert(&self) -> u32 {
        u32::from(self.edid[EDID_BYTE_MAX_CM_VERT])
    }
}
