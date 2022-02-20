use notedict::note_dict;

pub struct PwmSetting {
    pub div_int: u8,
    pub top: u16,
    pub top_pb: u16,
}

pub const NOTE_DICT: [PwmSetting; 128] = note_dict!();
