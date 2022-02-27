use midir::MidiOutputConnection;

use tide::Request;

fn play_note(mut conn_out: MidiOutputConnection, note: u8, duration: u64) {
    const NOTE_ON_MSG: u8 = 0x90;
    const NOTE_OFF_MSG: u8 = 0x80;
    const VELOCITY: u8 = 0x64;
    // We're ignoring errors in here
    let _ = conn_out.send(&[NOTE_ON_MSG, note, VELOCITY]);
    std::thread::sleep(std::time::Duration::from_millis(duration));
    let _ = conn_out.send(&[NOTE_OFF_MSG, note, VELOCITY]);

    conn_out.close();
}

async fn bonk(_req: Request<()>) -> tide::Result {
    let midi_out = midir::MidiOutput::new("Bonkmarmu Output")?;

    let out_ports = midi_out.ports();
    let out_port: &midir::MidiOutputPort = &out_ports[0];

    println!("\nOpening connection");
    let conn_out = midi_out.connect(out_port, "midir-test").unwrap();
    println!("Connection open. Listen!");

    println!("Bonking...");
    play_note(conn_out, 64, 2500);
    println!("Bonked");

    Ok("Bonk".into())
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Server listening to http://localhost:12389");

    let mut app = tide::new();
    app.at("/bonk").get(bonk);
    app.listen("0.0.0.0:12389").await?;

    Ok(())
}
