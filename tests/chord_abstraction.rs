use musicbox::TONIC;

#[test]
fn chord_abstraction() {
    use musicbox::notary::Notary;
    let mut maj7 = Notary::from(TONIC, 0);
    maj7.z0();
    maj7.z4();
    maj7.z7();
    maj7.y();

    assert_eq!([
                maj7.notary[0].round() as i64, 
                maj7.notary[1].round() as i64,
                maj7.notary[2].round() as i64,
                maj7.notary[3].round() as i64,
                ],          
                
                [27, 34, 40, 51])    
}
