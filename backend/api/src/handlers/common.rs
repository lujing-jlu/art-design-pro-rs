use actix_multipart::Multipart;
use actix_web::{post, web, HttpResponse, Responder};
use futures::StreamExt;
use serde_json::json;
use std::{fs, io::Write, path::PathBuf};

/// WangEditor 图片上传接口
///
/// 保存到 `./uploads` 目录，并返回可访问的 URL（由 actix-files 暴露）
#[post("/common/upload/wangeditor")]
pub async fn upload_wangeditor(
    upload_dir: web::Data<String>,
    mut payload: Multipart,
) -> impl Responder {
    let upload_dir = PathBuf::from(upload_dir.get_ref().as_str());
    if let Err(err) = fs::create_dir_all(&upload_dir) {
        return HttpResponse::Ok()
            .json(json!({ "errno": 1, "message": format!("创建上传目录失败: {}", err) }));
    }

    let mut saved_url: Option<String> = None;

    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(f) => f,
            Err(e) => {
                return HttpResponse::Ok()
                    .json(json!({ "errno": 1, "message": format!("读取上传流失败: {}", e) }));
            }
        };

        let filename = field
            .content_disposition()
            .get_filename()
            .map(|f| f.to_string())
            .unwrap_or_else(|| "upload.bin".to_string());

        let filepath = upload_dir.join(&filename);
        let mut f = match fs::File::create(&filepath) {
            Ok(file) => file,
            Err(err) => {
                return HttpResponse::Ok()
                    .json(json!({ "errno": 1, "message": format!("保存文件失败: {}", err) }));
            }
        };

        while let Some(chunk) = field.next().await {
            let data = match chunk {
                Ok(d) => d,
                Err(e) => {
                    return HttpResponse::Ok()
                        .json(json!({ "errno": 1, "message": format!("读取文件块失败: {}", e) }));
                }
            };
            if let Err(err) = f.write_all(&data) {
                return HttpResponse::Ok()
                    .json(json!({ "errno": 1, "message": format!("写入文件失败: {}", err) }));
            }
        }

        let url = format!("/uploads/{}", filename);
        saved_url = Some(url);
        break; // 仅处理第一个文件
    }

    if let Some(url) = saved_url {
        HttpResponse::Ok().json(json!({
            "errno": 0,
            "data": {
                "url": url,
                "alt": "upload",
                "href": url
            }
        }))
    } else {
        HttpResponse::Ok().json(json!({ "errno": 1, "message": "未收到文件" }))
    }
}
