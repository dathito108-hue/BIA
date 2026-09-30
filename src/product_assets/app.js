/* BIA Product Studio runtime — Apache-2.0; see LICENSE and NOTICE. */
'use strict';
(()=>{
 const $=id=>document.getElementById(id),p=PRODUCT,c=BiaCore,key='bia-product:'+p.id+':'+p.kind+':'+p.currency+':1';let rows=[],editing='',blocked=false;
 const labels={quote:['Bảng báo giá','Tổng trước thuế; không phải hóa đơn điện tử.'],stock:['Quản lý kho nhỏ','Số lượng hiện tại, chưa có đồng bộ hay sổ biến động kho.'],tasks:['Quản lý công việc','Theo dõi trên thiết bị này; chưa có nhắc việc nền.']};
 document.title=p.title;$('title').textContent=p.title;$('brief').textContent=p.brief;$('audience').textContent='Dành cho: '+p.audience;$('author').textContent='Người tạo: '+p.author;$('kindTitle').textContent=labels[p.kind][0];$('scope').textContent=labels[p.kind][1];
 // Validated color is data, never arbitrary CSS or HTML from the brief.
 document.documentElement.style.setProperty('--accent',p.accent);
 for(const [id,show] of Object.entries({sku:p.kind==='stock',qty:p.kind!=='tasks',price:p.kind==='quote',threshold:p.kind==='stock',due:p.kind==='tasks',priority:p.kind==='tasks',status:p.kind==='tasks'})){$(id+'Box').hidden=!show;$(id).disabled=!show;}
 function message(s){$('message').textContent=s;}
 function persist(next){if(blocked)throw Error('Dữ liệu lưu hỏng: khôi phục JSON đúng sản phẩm trước khi sửa');next=c.validate(next,p.kind,p.currency);localStorage.setItem(key,c.backup(next,p));rows=next;render();}
 function reset() {editing='';$('editor').reset();$('formTitle').textContent='Thêm mới';}
 function edit(r){editing=r.id;for(const k of ['name','note','qty','sku','threshold','due','priority','status'])if(r[k]!==undefined)$(k).value=r[k];if(r.price!==undefined)$('price').value=c.decimal(r.price,p.currency);$('formTitle').textContent='Chỉnh sửa';$('name').focus();}
 function td(tr,value){const cell=document.createElement('td');cell.textContent=value;tr.appendChild(cell);return cell;}
 function render(){
  $('rows').replaceChildren();$('thead').replaceChildren();const header=document.createElement('tr');const columns=p.kind==='quote'?['Tên','Số lượng','Đơn giá','Thành tiền','Ghi chú','Thao tác']:p.kind==='stock'?['Mã hàng','Tên','Tồn','Ngưỡng','Cảnh báo','Ghi chú','Thao tác']:['Tên','Hạn','Ưu tiên','Trạng thái','Ghi chú','Thao tác'];for(const h of columns){const th=document.createElement('th');th.textContent=h;header.appendChild(th);}$('thead').appendChild(header);
  const query=$('search').value.toLocaleLowerCase();const visible=rows.filter(r=>(r.name+' '+r.note+' '+(r.sku||'')).toLocaleLowerCase().includes(query));
  for(const r of visible){const tr=document.createElement('tr');const vals=p.kind==='quote'?[r.name,r.qty,c.money(r.price,p.currency),c.money((BigInt(r.price)*BigInt(r.qty)).toString(),p.currency),r.note]:p.kind==='stock'?[r.sku,r.name,r.qty,r.threshold,r.qty<=r.threshold?'Tồn thấp':'Đủ',r.note]:[r.name,r.due||'—',['','Cao','Vừa','Thấp'][r.priority],{todo:'Chưa làm',doing:'Đang làm',done:'Hoàn tất'}[r.status],r.note];for(const v of vals)td(tr,v);const actions=td(tr,'');for(const [label,fn] of [['Sửa',()=>edit(r)],['Xóa',()=>{if(confirm('Xóa dòng này?')){persist(rows.filter(x=>x.id!==r.id));if(editing===r.id)reset();}}]]){const b=document.createElement('button');b.textContent=label;b.className='secondary';b.onclick=()=>{try{fn();}catch(e){message(e.message);}};actions.appendChild(b);}$('rows').appendChild(tr);}
  $('empty').hidden=visible.length>0;$('empty').textContent=rows.length?'Không có kết quả phù hợp.':'Chưa có dữ liệu. Thêm dòng đầu tiên ở trên.';
  $('summary').textContent=p.kind==='quote'?'Tổng trước thuế: '+c.money(c.total(rows),p.currency):p.kind==='stock'?rows.length+' mã hàng · '+rows.filter(r=>r.qty<=r.threshold).length+' tồn thấp':rows.filter(r=>r.status==='done').length+'/'+rows.length+' công việc hoàn tất';
 }
 $('editor').onsubmit=e=>{e.preventDefault();try{const id=editing||(globalThis.crypto&&crypto.randomUUID?crypto.randomUUID():Date.now().toString(36)+'-'+Math.random().toString(36).slice(2));const r={id,name:$('name').value,note:$('note').value,qty:$('qty').value,price:p.kind==='quote'?c.minor($('price').value,p.currency):'0',sku:$('sku').value,threshold:$('threshold').value,due:$('due').value,priority:$('priority').value,status:$('status').value};const valid=c.row(r,p.kind,p.currency);persist(editing?rows.map(x=>x.id===editing?valid:x):[...rows,valid]);reset();message('Đã lưu trên thiết bị.');}catch(x){message(x.message);}};
 $('cancel').onclick=reset;$('search').oninput=render;
 function download(name,type,text){if(location.hostname==='bia-product.invalid'){message('Xem thử không tải file. Xuất ZIP rồi mở sản phẩm trong trình duyệt để sao lưu/CSV/in.');return;}const url=URL.createObjectURL(new Blob([text],{type})),a=document.createElement('a');a.href=url;a.download=name;document.body.appendChild(a);a.click();a.remove();setTimeout(()=>URL.revokeObjectURL(url),1000);}
 $('csv').onclick=()=>download('data.csv','text/csv;charset=utf-8',c.csv(rows,p.kind,p.currency));$('backup').onclick=()=>download('backup.json','application/json',c.backup(rows,p));
 $('restore').onchange=async e=>{try{const file=e.target.files[0];if(!file)return;if(file.size>1000000)throw Error('Tệp quá 1 MB');const next=c.restore(await file.text(),p);if(!confirm('Thay thế toàn bộ dữ liệu bằng '+next.length+' dòng trong bản sao lưu?'))return;localStorage.setItem(key,c.backup(next,p));blocked=false;rows=next;reset();render();message('Đã khôi phục.');}catch(x){message(x.message);}finally{e.target.value='';}};
 $('print').onclick=()=>{if(location.hostname==='bia-product.invalid')message('Mở gói ZIP bằng trình duyệt để in/PDF.');else window.print();};
 $('qa').onclick=()=>{try{message('Đạt '+c.selftest()+' kiểm tra tính toán/định dạng. Cần tự nghiệm thu thao tác, lưu/khôi phục trên thiết bị khách hàng.');}catch(e){message(e.message);}};
 window.addEventListener('storage',e=>{if(e.key===key){try{rows=e.newValue?c.restore(e.newValue,p):[];reset();render();message('Dữ liệu thay đổi ở tab khác.');}catch(x){blocked=true;message('Dữ liệu ở tab khác không hợp lệ.');}}});
 try{const raw=localStorage.getItem(key);rows=raw?c.restore(raw,p):[];}catch(e){blocked=true;message('Không đọc được dữ liệu đã lưu. Khôi phục JSON trước khi sửa; không tự ghi đè.');}
 render();window.BIA_READY=true;
})();
