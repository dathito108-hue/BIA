//! Bounded product compiler, not a general-purpose code model or proof of demand.
use crate::integrated_cognition::hex;
fn unhex(s: &str) -> Option<String> {
 if !s.len().is_multiple_of(2) || !s.is_ascii() { return None; }
 let bytes: Option<Vec<u8>> = s.as_bytes().as_chunks::<2>().0.iter().map(|p| u8::from_str_radix(std::str::from_utf8(p).ok()?,16).ok()).collect();
 String::from_utf8(bytes?).ok()
}


#[derive(Debug, Clone)]
pub struct Spec {
    pub id: String,
    pub kind: String,
    pub title: String,
    pub audience: String,
    pub brief: String,
    pub author: String,
    pub currency: String,
    pub accent: String,
}
impl Spec {
    pub fn parse(wire: &str) -> Result<Self, String> {
        if wire.len() > 16000 { return Err("Đặc tả quá dài".into()); }
        let lines: Vec<_> = wire.lines().collect();
        if lines.len()!=9 || lines[0]!="BIA_PRODUCT_1" { return Err("Sai định dạng đặc tả".into()); }
        let v: Vec<String> = lines[1..].iter().map(|s| unhex(s).ok_or_else(|| "Lỗi UTF-8/hex".to_string())).collect::<Result<_,_>>()?;
        let s=Self{id:v[0].clone(),kind:v[1].clone(),title:v[2].clone(),audience:v[3].clone(),brief:v[4].clone(),author:v[5].clone(),currency:v[6].clone(),accent:v[7].clone()};
        s.validate()?; Ok(s)
    }
    pub fn validate(&self) -> Result<(),String> {
        if !(8..=64).contains(&self.id.len()) || !self.id.bytes().all(|c|c.is_ascii_alphanumeric()||c==b'-') {return Err("ID không hợp lệ".into());}
        if !["quote","stock","tasks"].contains(&self.kind.as_str()) {return Err("Chỉ hỗ trợ báo giá, kho hoặc công việc".into());}
        for (value,limit) in [(&self.title,80),(&self.audience,240),(&self.brief,2000),(&self.author,120)] {
            if value.trim().is_empty() || value.chars().count()>limit || value.chars().any(char::is_control) {return Err("Điền đủ tên, khách hàng, yêu cầu và tác giả; không dùng ký tự điều khiển".into());}
        }
        if !["VND","USD","EUR"].contains(&self.currency.as_str()) || self.accent.len()!=7 || !self.accent.starts_with('#') || !self.accent[1..].bytes().all(|b|b.is_ascii_hexdigit()) {return Err("Tiền tệ/màu không hợp lệ".into());}
        Ok(())
    }
    pub fn wire(&self)->String {std::iter::once("BIA_PRODUCT_1".into()).chain([&self.id,&self.kind,&self.title,&self.audience,&self.brief,&self.author,&self.currency,&self.accent].iter().map(|s|hex(s.as_bytes()))).collect::<Vec<_>>().join("\n")}
}
pub fn json(s:&str)->String {
    let mut out=String::from("\"");
    for c in s.chars(){match c {'"'=>out.push_str("\\\""),'\\'=>out.push_str("\\\\"),'\n'=>out.push_str("\\n"),'\r'=>out.push_str("\\r"),'\t'=>out.push_str("\\t"),'<'|'>'|'&'|'\u{2028}'|'\u{2029}'=>out.push_str(&format!("\\u{:04x}",c as u32)),c if c.is_control()=>out.push_str(&format!("\\u{:04x}",c as u32)),_=>out.push(c)}}
    out.push('"');out
}
pub fn compile(s:&Spec)->Result<Vec<(String,String)>,String> {
    s.validate()?;
    let config=format!("'use strict';\nconst PRODUCT=Object.freeze({{id:{},kind:{},title:{},audience:{},brief:{},author:{},currency:{},accent:{}}});\n",json(&s.id),json(&s.kind),json(&s.title),json(&s.audience),json(&s.brief),json(&s.author),json(&s.currency),json(&s.accent));
    let description=match s.kind.as_str(){"quote"=>"Bảng báo giá: dòng hàng, số lượng nguyên, đơn giá, tổng trước thuế; chưa phải hóa đơn điện tử.","stock"=>"Kho nhỏ: mã hàng không trùng, số lượng hiện tại, ngưỡng tồn thấp và ghi chú. Không có đồng bộ nhiều người hay sổ kiểm toán biến động kho.",_=>"Công việc: tiêu đề, hạn hoàn thành, ưu tiên, trạng thái và ghi chú. Không có tài khoản hoặc nhắc việc nền."};
    let readme=format!("# {}\n\n{}\n\n## Chạy\nGiải nén toàn bộ gói. Mở index.html bằng trình duyệt hiện đại có JavaScript/BigInt. Có thể phục vụ thư mục bằng máy chủ tĩnh; không cần npm/backend hoặc BIA. Nếu trình duyệt không cho lưu localStorage với file://, dùng máy chủ tĩnh và sao lưu JSON. Không đổi origin/đường dẫn triển khai khi chưa sao lưu.\n\n## Dữ liệu\nDữ liệu lưu trong trình duyệt trên thiết bị này; tối đa 500 dòng. Không được gửi lên mạng. Xóa dữ liệu trình duyệt sẽ mất dữ liệu chưa sao lưu. Xuất JSON định kỳ; nhập JSON thay thế toàn bộ dữ liệu sau xác nhận. CSV dùng cho bàn giao/bảng tính; không hỗ trợ nhập CSV. Xem thử trong BIA chặn tải file/in; hãy xuất ZIP và mở bằng trình duyệt để dùng đầy đủ.\n\n## Tùy biến\nconfig.js chứa cấu hình. app.js, core.js và style.css là mã nguồn. Giữ product id để tiếp tục dữ liệu. Đổi loại/tiền tệ sẽ dùng vùng dữ liệu riêng; không tự quy đổi số tiền. Một mã hàng = một dòng; số lượng nguyên 0–1.000.000; đơn giá tối đa 1.000.000.000.000 đơn vị tiền tệ. Tổng báo giá chưa cộng thuế/chiết khấu.\n\n## Kiểm tra và bàn giao\nMở kiểm tra tích hợp, thử thêm/sửa/xóa/tìm kiếm, tải lại trang, xuất/nhập JSON, CSV và in trên thiết bị khách hàng. QA.json chỉ mô tả kiểm tra sinh mã, không thay thế nghiệm thu runtime. Không có tài khoản, thanh toán, đồng bộ hay tự triển khai.\n\n## Quyền sử dụng\nMã nền sinh từ BIA theo Apache-2.0; giữ LICENSE và NOTICE khi phân phối. Không cam kết quyền độc quyền mã nền. Tên/nội dung do người tạo cung cấp.\n",s.title,description);
    let sales=format!("# Bản nháp giới thiệu — cần người bán rà soát\n\nTên: {}\nKhách hàng giả định: {}\nVấn đề theo yêu cầu: {}\nNgười tạo: {}\n\nPhạm vi thực tế: {}\nGói bàn giao: mã HTML/CSS/JS, cấu hình, hướng dẫn và mã kiểm tra.\n\nChưa kiểm chứng nhu cầu, giá bán, khách hàng hoặc doanh thu. Chưa đăng bán.\n\n## Phiếu định giá (người bán điền)\nGiờ tùy biến: ___\nChi phí/giờ: ___\nChi phí hỗ trợ/triển khai: ___\nGiá đề xuất và phạm vi sửa: ___\nNgày/thiết bị khách hàng nghiệm thu: ___\n\nKhông hứa tính năng ngoài README. Nếu cần tài khoản, nhiều người dùng hay thanh toán, phải phát triển và kiểm thử thêm trước khi nhận cam kết.\n",s.title,s.audience,s.brief,s.author,description);
    let mut files: Vec<(String,String)>=vec![("index.html".into(),include_str!("product_assets/index.html").into()),("style.css".into(),include_str!("product_assets/style.css").into()),("core.js".into(),include_str!("product_assets/core.js").into()),("app.js".into(),include_str!("product_assets/app.js").into()),("config.js".into(),config),("README.md".into(),readme),("SALES-DRAFT.md".into(),sales),("SPEC.bia".into(),s.wire()),("LICENSE".into(),include_str!("../LICENSE").into()),("NOTICE".into(),"Generated by BIA Product Studio V138. Product runtime derived from BIA (Apache-2.0). Preserve LICENSE and this notice when redistributing. User-supplied names and descriptions are not endorsements.\n".into()),("QA.json".into(),"{\"schema\":\"BIA_PRODUCT_QA_1\",\"spec_validation\":true,\"generator\":\"V138\",\"runtime_acceptance\":\"not_yet_verified_for_this_product\",\"market_validation\":false,\"published\":false}".into())];
    files.sort_by(|a,b|a.0.cmp(&b.0));Ok(files)
}
pub fn compile_wire(wire:&str)->String {match Spec::parse(wire).and_then(|s|compile(&s)){Ok(files)=>std::iter::once("BIA_BUNDLE_1".into()).chain(files.into_iter().map(|(path,body)|format!("{}\t{}",path,hex(body.as_bytes())))).collect::<Vec<_>>().join("\n"),Err(e)=>format!("ERROR\t{}",hex(e.as_bytes()))}}
