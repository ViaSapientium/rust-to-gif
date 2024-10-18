use ffmpeg_next::{codec, format, frame, media, software, util};
use std::error::Error;
use std::path::Path;

/// Fonction pour extraire les images de la vidéo
pub fn extract_images(video_path: &str, output_dir: &str) -> Result<(), Box<dyn Error>> {
  // Initialiser FFmpeg
  ffmpeg_next::init().unwrap();

  // Ouvrir le fichier vidéo
  let mut ictx = format::input(&video_path).unwrap();
  let input = ictx.streams().best(media::Type::Video).unwrap();
  let video_stream_index = input.index();
  let mut decoder = codec::context::Context::from_parameters(input.codec())?
    .decoder()
    .video()?;

  // Configurer le scaler pour convertir les frames en RGB
  let mut scaler = software::scaling::Context::get(
    decoder.format(),
    decoder.width(),
    decoder.height(),
    util::frame::PixelFormat::RGB24,
    decoder.width(),
    decoder.height(),
    software::scaling::Flags::BILINEAR,
  )?;

  let mut frame_index = 0;
  let mut decoded = frame::Video::empty();

  // Boucler à travers les packets de la vidéo
  for (stream, packet) in ictx.packets() {
    if stream.index() == video_stream_index {
      decoder.decode(&packet, &mut decoded)?;

      // Convertir le frame en RGB
      let mut rgb_frame = frame::Video::empty();
      scaler.run(&decoded, &mut rgb_frame)?;

      // Sauvegarder une image toutes les 5 secondes (à ajuster selon FPS)
      if frame_index % 5 == 0 {
        let file_name = format!("{}/frame_{}.png", output_dir, frame_index);
        save_frame_as_image(&rgb_frame, &file_name)?;
      }
      frame_index += 1;
    }
  }

  Ok(())
}

/// Fonction pour sauvegarder un frame en tant qu'image PNG
fn save_frame_as_image(frame: &frame::Video, file_name: &str) -> Result<(), Box<dyn Error>> {
  let mut buf = vec![0; frame.stride(0) * frame.height() as usize];
  frame.copy_plane(0, &mut buf, frame.stride(0))?;

  let path = Path::new(file_name);
  let file = std::fs::File::create(path)?;
  let mut encoder = png::Encoder::new(file, frame.width() as u32, frame.height() as u32);
  encoder.set_color(png::ColorType::Rgb);
  encoder.set_depth(png::BitDepth::Eight);

  let mut writer = encoder.write_header()?;
  writer.write_image_data(&buf)?;

  Ok(())
}
