use musicbox::makam::Makam;

fn main() {
    let mut cargah = Makam::new();
    cargah.cargah();
    dbg!(&cargah.djin[1]);
}
