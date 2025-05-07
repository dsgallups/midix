use midix::PitchBend;

#[test]
fn create_pitch_bend_event() {
    let pb = PitchBend::from_f32(0.0);
    assert_eq!(pb.value(), PitchBend::MID_BYTES);
    let pb = PitchBend::from_f64(0.0);
    assert_eq!(pb.value(), PitchBend::MID_BYTES);
    let pb = PitchBend::from_int(0);
    assert_eq!(pb.value(), PitchBend::MID_BYTES);
}
#[test]
fn create_negative_pitch_bend_event() {
    let pb = PitchBend::from_f32(-0.1);
    assert_eq!(
        pb.value(),
        PitchBend::MID_BYTES - (PitchBend::MID_BYTES as f32 * 0.1) as u16
    );
    let pb = PitchBend::from_f64(-0.1);
    assert_eq!(
        pb.value(),
        PitchBend::MID_BYTES - (PitchBend::MID_BYTES as f32 * 0.1) as u16
    );
    let pb = PitchBend::from_int(-82);
    assert_eq!(pb.value(), PitchBend::MID_BYTES - 82);
}
#[test]
fn pitch_bend_round_trip() {
    for value in 0..PitchBend::MAX_VALUE {
        let pb = PitchBend::from_u16(value);
        assert_eq!(pb.value(), value);
        let lsb = pb.lsb();
        let msb = pb.msb();
        let pb2 = PitchBend::from_bits_unchecked((lsb as u16) << 8 | msb as u16);
        assert_eq!(pb2.value(), value);
    }
    assert_eq!(PitchBend::from_f32(0.5).as_f32(), 0.5);
    assert_eq!(PitchBend::from_f32(0.875).as_f32(), 0.875);
    assert_eq!(PitchBend::from_f32(-1.1).as_f32(), -1.0);
    assert_eq!(PitchBend::from_f64(-0.5).as_f64(), -0.5);
    assert_eq!(PitchBend::from_int(-200).as_int(), -200);
}
