/* BIA Product Studio runtime — Apache-2.0; see LICENSE and NOTICE. */
'use strict';
const BiaCore=(()=>{
 const limit=500;
 function text(s,n,empty=false){if(typeof s!=='string'||s.length>n||(!empty&&!s.trim())||/[\u0000-\u001f\u007f]/.test(s))throw Error('Nội dung không hợp lệ');return s.trim();}
 function integer(v,max=1000000){if(!/^\d+$/.test(String(v)))throw Error('Cần số nguyên không âm');const n=Number(v);if(!Number.isSafeInteger(n)||n>max)throw Error('Số vượt giới hạn');return n;}
 function minor(v,currency){const d=currency==='VND'?0:2;const re=d?/^\d{1,13}(?:\.\d{1,2})?$/:/^\d{1,13}$/;if(!re.test(String(v)))throw Error('Đơn giá không hợp lệ; dùng dấu chấm thập phân');const [a,b='']=String(v).split('.');const n=BigInt(a)*10n**BigInt(d)+BigInt(b.padEnd(d,'0')||'0');if(n>1000000000000n*10n**BigInt(d))throw Error('Đơn giá quá lớn');return n.toString();}
 function decimal(v,currency){const d=currency==='VND'?0:2;let s=BigInt(v).toString().padStart(d+1,'0');return d?s.slice(0,-d)+'.'+s.slice(-d):s;}
 function money(v,currency){const [a,b]=decimal(v,currency).split('.');return a.replace(/\B(?=(\d{3})+(?!\d))/g,',')+(b===undefined?'':'.'+b)+' '+currency;}
 function date(s){if(s==='')return s;if(!/^\d{4}-\d\d-\d\d$/.test(s))throw Error('Ngày không hợp lệ');const d=new Date(s+'T00:00:00Z');if(!Number.isFinite(d.getTime())||d.toISOString().slice(0,10)!==s)throw Error('Ngày không tồn tại');return s;}
 function row(r,kind,currency){
  if(!r||typeof r!=='object'||Array.isArray(r))throw Error('Dòng dữ liệu không hợp lệ');
  const x={id:text(r.id,80),name:text(r.name,120),note:text(r.note,500,true)};
  if(!/^[a-zA-Z0-9-]+$/.test(x.id))throw Error('ID không hợp lệ');
  if(kind==='tasks'){x.due=date(r.due);x.priority=integer(r.priority,3);if(x.priority<1||!['todo','doing','done'].includes(r.status))throw Error('Trạng thái/ưu tiên sai');x.status=r.status;}
  else {x.qty=integer(r.qty);if(kind==='stock'){x.sku=text(r.sku,60);x.threshold=integer(r.threshold);}else if(kind==='quote'){if(typeof r.price!=='string'||!/^\d{1,15}$/.test(r.price))throw Error('Đơn giá sai');x.price=minor(decimal(r.price,currency),currency);}else throw Error('Loại sản phẩm sai');}
  return x;
 }
 function validate(rows,kind,currency){if(!Array.isArray(rows)||rows.length>limit)throw Error('Tối đa 500 dòng');const out=rows.map(r=>row(r,kind,currency));const ids=new Set(),skus=new Set();for(const r of out){if(ids.has(r.id))throw Error('Trùng ID');ids.add(r.id);if(kind==='stock'){const key=r.sku.toLocaleLowerCase();if(skus.has(key))throw Error('Trùng mã hàng');skus.add(key);}}return out;}
 function total(rows){return rows.reduce((s,r)=>s+BigInt(r.price)*BigInt(r.qty),0n).toString();}
 function cell(s){s=String(s);if(/^[\s]*[=+\-@]/.test(s)||/^[\t\r]/.test(s))s="'"+s;return '"'+s.replace(/"/g,'""')+'"';}
 function csv(rows,kind,currency){const columns=kind==='tasks'?['name','due','priority','status','note']:kind==='stock'?['sku','name','qty','threshold','note']:['name','qty','price','note'];return '\ufeff'+[columns,...rows.map(r=>columns.map(k=>k==='price'?decimal(r[k],currency):r[k]))].map(a=>a.map(cell).join(',')).join('\r\n');}
 function backup(rows,p){return JSON.stringify({schema:'BIA_PRODUCT_DATA_1',id:p.id,kind:p.kind,currency:p.currency,rows:validate(rows,p.kind,p.currency)},null,2);}
 function restore(raw,p){if(typeof raw!=='string'||raw.length>1000000)throw Error('Tệp quá lớn');const x=JSON.parse(raw);if(x.schema!=='BIA_PRODUCT_DATA_1'||x.id!==p.id||x.kind!==p.kind||x.currency!==p.currency)throw Error('Bản sao lưu không khớp sản phẩm/tiền tệ');return validate(x.rows,p.kind,p.currency);}
 function selftest(){let count=0;const check=(x)=>{if(!x)throw Error('Kiểm tra thất bại');count++;};check(minor('1.25','USD')==='125');check(total([{price:'99999999999999',qty:1000000}])==='99999999999999000000');check(csv([{name:'=1+1',qty:1,price:'100',note:''}],'quote','VND').includes("'=1+1"));try{date('2026-02-30');throw Error('accepted');}catch(e){check(e.message!=='accepted');}return count;}
 return Object.freeze({integer,minor,decimal,money,row,validate,total,csv,backup,restore,selftest});
})();
if(typeof module!=='undefined')module.exports=BiaCore;
