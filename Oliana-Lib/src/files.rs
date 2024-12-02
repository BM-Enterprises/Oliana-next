
use crate as oliana_lib; // This helps our crate::err::eloc!() leak state via a struct

pub async fn existinate(
  local_file_path: impl Into<std::path::PathBuf>,
  remote_download_url: &str
) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
  let local_file_path = local_file_path.into();

  if !tokio::fs::try_exists(&local_file_path).await? {
    eprintln!("Downloading {} to {}", remote_download_url, &local_file_path.to_string_lossy() );
    if remote_download_url.len() < 1 {
      return Err(format!("The file {:?} does not exist and no URL was passed to download it!", &local_file_path).into());
    }

    let mut downloader = downloader::Downloader::builder()
          .download_folder( local_file_path.parent().ok_or_else(|| return "No Parent Directory for passed file to be downloaded!" ).map_err(crate::err::eloc!())? )
          .parallel_requests(2)
          .build()?;
    let dl_file_name_osstr = local_file_path.file_name().ok_or_else(|| return "No File Name for passed file to be downloaded!" ).map_err(crate::err::eloc!())?;
    let dl_file_name_string = dl_file_name_osstr.to_string_lossy().into_owned();

    let dl = downloader::Download::new(remote_download_url)
                .file_name( &std::path::Path::new( &dl_file_name_string ) )
                .progress(std::sync::Arc::new(
                  DownloadProgressReporter::new()
                ));

    let _result = downloader.async_download(&[dl]).await?;

  }
  else {
    eprintln!("Found already-downloaded file {}", &local_file_path.to_string_lossy() );
  }

  Ok(local_file_path)
}



pub struct DownloadProgressReporter {
    pub max_progress: std::cell::UnsafeCell<std::option::Option<u64>>,
    pub bar: indicatif::ProgressBar,
}

unsafe impl Sync for DownloadProgressReporter { } // Because I said so, our UnsafeCell is just a number in memory

impl DownloadProgressReporter {
    pub fn new() -> Self {
        Self {
            max_progress: None.into(),
            bar: indicatif::ProgressBar::no_length()
        }
    }
}

impl Drop for DownloadProgressReporter {
    fn drop(&mut self) {
        self.bar.finish();
    }
}


impl downloader::progress::Reporter for DownloadProgressReporter {
    fn setup(&self, max_progress: std::option::Option<u64>, message: &str) {
        unsafe { *self.max_progress.get() = max_progress.into(); } // Assigns into a read-only reference; safe because I say the compiler won't optimize through an UnsafeCell
        if let Some(max_progress_val) = max_progress {
            self.bar.set_length(max_progress_val);
        }
    }
    fn progress(&self, current: u64) {
        if current > self.bar.position() {
            let incr_amnt = current - self.bar.position();
            self.bar.inc(incr_amnt);
        }
    }
    fn set_message(&self, message: &str) {

    }
    fn done(&self) {
        self.bar.finish();
    }
}



