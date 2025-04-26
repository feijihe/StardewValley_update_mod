use std::path::Path;
use tokio::fs;
use std::io;
use reqwest::{self, header};
use regex::Regex;
use anyhow::{Context, Result};

const COOKIE: &str = "_pk_id.1.3564=4a3158354c1f2ef7.1728268898.; __qca=P0-754831997-1728268896934; _hjSessionUser_1264276=eyJpZCI6IjVmNzgzNTExLTM2NDctNTY4MS1hYTgzLTJlZjE2YTZjZmFmYyIsImNyZWF0ZWQiOjE3MjgyNjg5MDA0NjksImV4aXN0aW5nIjp0cnVlfQ==; _sp_su=false; usnatUUID=30c3fe54-1dd0-4e97-8e7b-7d635c034e25; _sharedId=52e21499-d5b3-41a7-aed8-94f771195df7; _cc_id=cc87dcf0ca121422f5a6ed42c6245f54; _au_1d=AU1D-0100-001730787152-SPMAR1G6-IC37; _au_last_seen_iab_tcf=1730787152823; idw-fe-id=12ff254c-c0d0-4c42-8ad9-c1d402f7cf5e; __gads=ID=66462a95de6d1fee:T=1730788710:RT=1730789346:S=ALNI_MZSlwkNIWUGKh2yp9uKzhG0b2lGAg; __gpi=UID=00000f60d350f8e0:T=1730788710:RT=1730789346:S=ALNI_Ma_mvQpZ0x7DKWvMZ2uGFI8DLG1sg; __eoi=ID=07d1b5b095ec42b5:T=1730788710:RT=1730789346:S=AA-AfjYzHCGxDFb4O9OMpIrg7rUA; _ga_FVWZ0RM4DH=GS1.1.1730787157.1.1.1730789516.60.0.0; _sharedId_cst=VyxHLMwsHQ%3D%3D; cto_bidid=UP1joF93MHJkejE4RlVMa2dsSWNqeTRBWjJ6MGdNSlN5VnAxWDNJaCUyRnlaYyUyQllrQU1rQU1UZkZiNUg0eCUyRmx2NDlxSUZTZVAlMkZBN3hPbWJnMWRRN2p2c1JuZG96aWlrMTB6JTJCbUhlWUMzbGFuTlBaJTJCYyUzRA; cto_dna_bundle=0MOZ9F9KcFFCSnF2cjB5VjdiaW5NWnZXMElUY0FNQ1JwJTJGaTBlZk82MEJ5SDU2azRBODBjd1hvdk1zTDFqeHRmQ3Z4ODF4YjRHZTJzTTglMkYzc2dIeUYya1BXdlElM0QlM0Q; fwroute=1745656964.89.16601.104819|e8cf353e01a4dded0b238db3d1674aad; __cflb=02DiuEJ6HvyJbNTdkTXmMfoybo4Ng2vHUC1bcb8qnqN5S; _gid=GA1.2.1128952170.1745656965; _pk_ref.3.3564=%5B%22%22%2C%22%22%2C1745656970%2C%22https%3A%2F%2Fwww.bing.com%2F%22%5D; _pk_id.3.3564=072918a1eda698bf.1745656970.; _pk_ses.3.3564=1; _hjSession_1264276=eyJpZCI6ImY4NjU2MTJmLTBkZjgtNGJmYi05M2IyLTk2Yzc1Y2YzMTNlZiIsImMiOjE3NDU2NTY5NzUzNzMsInMiOjAsInIiOjAsInNiIjowLCJzciI6MCwic2UiOjAsImZzIjowLCJzcCI6MH0=; _sharedid=4ed7d185-4b70-4dd2-ad5c-d7519076272b; _sharedid_cst=zix7LPQsHA%3D%3D; nexusmods_session=52a25220dde9f4cdd5090160dfdecbaa; nexusmods_session_refresh=1745657011; _hjHasCachedUserAttributes=true; bounceClientVisit6970v=N4IgNgDiBcIBYBcEQM4FIDMBBNAmAYnvgO6kB0AdgKYAeArigLYD2AJimQMbONEDmAQ0ZV0BFAgEAnVlWIA3AWDBUAniAA0ISTBAMqkjtXpM2Hbow0gAligD6fZrZQiUV5hRgAzRc8037EE4ubh7Q3mDOAL5AA; _lr_geo_location=HK; cto_bundle=gMOED19KcFFCSnF2cjB5VjdiaW5NWnZXMElhOUVkYlZPczNRMkhsdENjTDBYUkFDUHRUUmQlMkI1SFp6MzlKMTJyWmFyUTRWRmdQNkx6V04lMkZxQkNYTTBJTk5SQjEzNzJRMlklMkZIeHBTTzJyTHpuN0U5WFExSXdrY1FQZjh6U1RJZlFPOCUyRm1xbkh4WGdvdFNST1lyMWZHSWFoQmt3USUzRCUzRA; recentSearches=%5B%22automate%22%5D; bounceClientVisit6970=N4Igzg7gbiBcBmBDANmApgGhAEwJbbhAFYAGIgdgEYA2AThMstMoA4iWAWFlkLKfOJXIci1CmSFFa1AMzks8KABdB5GSXItaRAEw6siAPZwSWZAAdCACyVLzYAKQyAgg50AxN+4g+AdADs0AA8AVzAAW0NsMF8AY0NwrwBzRHC0Rw8wJUQAJ2w0aBRkNABPXhBEMBM+cxhYDiwCWBxynMIwtByYwNCIqJj48PLY5VURMXIJLDABWEYsJNi22HYsIdhTEGRRueFRdhkOSixY1PNEXCT-KthQHQ4ZRiY4UG2VXfGDo7NEfibKAC+WHuj0oOkoLy2OyEnxYh2OW1+swRUDqgKBICssTAAH1EAAjIJKdLvADaAF0AUA; _ga_0CPE0JFSCT=GS1.1.1745656986.2.1.1745658358.0.0.0; ab=0|1745658660; _pk_ref.1.3564=%5B%22%22%2C%22%22%2C1745658360%2C%22https%3A%2F%2Fusers.nexusmods.com%2F%22%5D; _pk_ses.1.3564=1; _ga_N0TELNQ37M=GS1.1.1745656965.32.1.1745658360.0.0.0; _ga=GA1.1.1966229069.1728268897; cf_clearance=BU2cGvXXmgcaSFcqqn4gDEaaXttaSlQNDuIDoKPjjEo-1745658361-1.2.1.1-7Tm7L0SeM6vHfoEAoV.aDS7YpaYmIPjFs5oZ7L1llXefwfbJEy6SyBuNOEbqhPc1w2tfPjNj.I7i9lDv2e6A8SH2CYXn5ZY5vDirg_7fwTQDQ7y122_2Y8pbge364pIm_8EGSU2xtzkL7CH5OIhmYlxbrVXSH0nH0NWBt4cnxEykip2VHHe60f6hw9d1iq1EmlLNbqOOclIThBckMqRlV0_THAPFCut_ldnm9VXC_McBfYyu80hqQihjRbDZsFy8NsVzh4SFfcqBKUmwhPYQ3IAukuZ8cho9I9qL6AcO79R2d77xUy.mdcS_c.zsOY5.YIljq.IEAulyeYSBI4AdoN5eF4Vmfw.K947Sa5Xfbok";

pub async fn get_mod_manifest(path: &str) -> Result<String, io::Error> {
    let mainfest_path = Path::new(path).join("manifest.json");

    match fs::read_to_string(&mainfest_path).await {
        Ok(content) => Ok(content),
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            let mut entries = fs::read_dir(path).await?;

            while let Some(entry) = entries.next_entry().await? {
               let entry_path = entry.path();
               if entry_path.is_dir() {
                   let result = Box::pin(get_mod_manifest(entry_path.to_str().unwrap())).await;
                   if result.is_ok() {
                       return result;
                   }
               } 
            }
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "manifest.json not found in the directory or its subdirectories"
            ))
        }
        Err(e) => Err(e), 
    }
}

pub async fn get_mod_latest_version(mod_id: &str) -> Result<(String, String)> {
    // 这里需要使用 nexusmods 的 API 来获取 mod 的最新版本号 
    // 具体实现可以参考 nexusmods 的 API 文档
    // let client = reqwest::Client::builder().timeout(std::time::Duration::from_secs(3)).build().unwrap();
    let client = reqwest::Client::new();
    let url = format!("https://www.nexusmods.com/stardewvalley/mods/{}?tab=files", mod_id);

    let response = client
        .get(&url)
        .header(header::COOKIE, COOKIE)
        .header("x-requested-with", "XMLHttpRequest")
        .header("Host", "www.nexusmods.com")
        .send()
        .await
        .context("Failed to send request")?;


    let body = response.text().await.context("Failed to read response body")?;

    let re = Regex::new(r#"<dt id=".*?" class=".*?" data-id="(.*?)" data-name=".*?" data-size="\d+" data-version="v?(.*?)" data-date="\d+">"#)
        .context("Invalid regex pattern")?;

    let captures = re.captures(&body)
        .context("Failed to find version information")?;

    let fid = captures.get(1)
        .context("Missing fid capture group")?
        .as_str()
        .to_string();
    
    let version = captures.get(2)
        .context("Missing version capture group")?
        .as_str()
        .to_string();

    Ok((fid, version))
}
