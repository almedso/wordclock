let U=0,P=null,R=`undefined`,Y=`boolean`,N=128,Z=`string`,Q=1,_=`Object`,S=`utf-8`,X=`number`,$=4,W=`function`,M=Array,T=Error,a0=Object,V=Uint8Array,O=undefined;var G=(async(a,b)=>{if(typeof Response===W&&a instanceof Response){if(typeof WebAssembly.instantiateStreaming===W){try{return await WebAssembly.instantiateStreaming(a,b)}catch(b){if(a.headers.get(`Content-Type`)!=`application/wasm`){console.warn(`\`WebAssembly.instantiateStreaming\` failed because your server does not serve wasm with \`application/wasm\` MIME type. Falling back to \`WebAssembly.instantiate\` which is slower. Original error:\\n`,b)}else{throw b}}};const c=await a.arrayBuffer();return await WebAssembly.instantiate(c,b)}else{const c=await WebAssembly.instantiate(a,b);if(c instanceof WebAssembly.Instance){return {instance:c,module:a}}else{return c}}});var u=(a=>{const b=typeof a;if(b==X||b==Y||a==P){return `${a}`};if(b==Z){return `"${a}"`};if(b==`symbol`){const b=a.description;if(b==P){return `Symbol`}else{return `Symbol(${b})`}};if(b==W){const b=a.name;if(typeof b==Z&&b.length>U){return `Function(${b})`}else{return `Function`}};if(M.isArray(a)){const b=a.length;let c=`[`;if(b>U){c+=u(a[U])};for(let d=Q;d<b;d++){c+=`, `+ u(a[d])};c+=`]`;return c};const c=/\[object ([^\]]+)\]/.exec(toString.call(a));let d;if(c.length>Q){d=c[Q]}else{return toString.call(a)};if(d==_){try{return `Object(`+ JSON.stringify(a)+ `)`}catch(a){return _}};if(a instanceof T){return `${a.name}: ${a.message}\n${a.stack}`};return d});var A=((b,c,d)=>{a._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h502a691ce059cac5(b,c,g(d))});var I=((a,b)=>{});var F=((a,b)=>{a=a>>>U;const c=E();const d=c.subarray(a/$,a/$+ b);const e=[];for(let a=U;a<d.length;a++){e.push(f(d[a]))};return e});var g=(a=>{if(d===b.length)b.push(b.length+ Q);const c=d;d=b[c];b[c]=a;return c});var f=(a=>{const b=c(a);e(a);return b});var L=(async(b)=>{if(a!==O)return a;if(typeof b===R){b=new URL(`yew-word-clock-a3331c52b8ab5ccf_bg.wasm`,import.meta.url)};const c=H();if(typeof b===Z||typeof Request===W&&b instanceof Request||typeof URL===W&&b instanceof URL){b=fetch(b)};I(c);const {instance:d,module:e}=await G(await b,c);return J(d,e)});var K=(b=>{if(a!==O)return a;const c=H();I(c);if(!(b instanceof WebAssembly.Module)){b=new WebAssembly.Module(b)};const d=new WebAssembly.Instance(b,c);return J(d,b)});function C(b,c){try{return b.apply(this,c)}catch(b){a.__wbindgen_exn_store(g(b))}}var z=((c,d,e)=>{try{a._dyn_core__ops__function__FnMut___A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h56dd8d6140585c68(c,d,y(e))}finally{b[x++]=O}});var J=((b,c)=>{a=b.exports;L.__wbindgen_wasm_module=c;s=P;q=P;D=P;i=P;a.__wbindgen_start();return a});var r=(()=>{if(q===P||q.byteLength===U){q=new Int32Array(a.memory.buffer)};return q});var p=(a=>a===O||a===P);var c=(a=>b[a]);var w=((b,c)=>{a.wasm_bindgen__convert__closures__invoke0_mut__hc2a281a137a3fe70(b,c)});var H=(()=>{const b={};b.wbg={};b.wbg.__wbindgen_object_drop_ref=(a=>{f(a)});b.wbg.__wbindgen_cb_drop=(a=>{const b=f(a).original;if(b.cnt--==Q){b.a=U;return !0};const c=!1;return c});b.wbg.__wbindgen_object_clone_ref=(a=>{const b=c(a);return g(b)});b.wbg.__wbindgen_string_new=((a,b)=>{const c=k(a,b);return g(c)});b.wbg.__wbg_listenerid_12315eee21527820=((a,b)=>{const d=c(b).__yew_listener_id;r()[a/$+ Q]=p(d)?U:d;r()[a/$+ U]=!p(d)});b.wbg.__wbg_setlistenerid_3183aae8fa5840fb=((a,b)=>{c(a).__yew_listener_id=b>>>U});b.wbg.__wbg_subtreeid_e348577f7ef777e3=((a,b)=>{const d=c(b).__yew_subtree_id;r()[a/$+ Q]=p(d)?U:d;r()[a/$+ U]=!p(d)});b.wbg.__wbg_setsubtreeid_d32e6327eef1f7fc=((a,b)=>{c(a).__yew_subtree_id=b>>>U});b.wbg.__wbg_cachekey_b61393159c57fd7b=((a,b)=>{const d=c(b).__yew_subtree_cache_key;r()[a/$+ Q]=p(d)?U:d;r()[a/$+ U]=!p(d)});b.wbg.__wbg_setcachekey_80183b7cfc421143=((a,b)=>{c(a).__yew_subtree_cache_key=b>>>U});b.wbg.__wbg_new_abda76e883ba8a5f=(()=>{const a=new T();return g(a)});b.wbg.__wbg_stack_658279fe44541cf6=((b,d)=>{const e=c(d).stack;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ Q]=g;r()[b/$+ U]=f});b.wbg.__wbg_error_f851667af71bcfc6=((b,c)=>{let d;let e;try{d=b;e=c;console.error(k(b,c))}finally{a.__wbindgen_free(d,e,Q)}});b.wbg.__wbg_clearInterval_bd072ecb096d9775=(a=>{const b=clearInterval(f(a));return g(b)});b.wbg.__wbg_setInterval_edede8e2124cbb00=function(){return C(((a,b)=>{const d=setInterval(c(a),b);return g(d)}),arguments)};b.wbg.__wbg_queueMicrotask_118eeb525d584d9a=(a=>{queueMicrotask(c(a))});b.wbg.__wbg_queueMicrotask_26a89c14c53809c0=(a=>{const b=c(a).queueMicrotask;return g(b)});b.wbg.__wbindgen_is_function=(a=>{const b=typeof c(a)===W;return b});b.wbg.__wbindgen_error_new=((a,b)=>{const c=new T(k(a,b));return g(c)});b.wbg.__wbindgen_string_get=((b,d)=>{const e=c(d);const f=typeof e===Z?e:O;var g=p(f)?U:o(f,a.__wbindgen_malloc,a.__wbindgen_realloc);var h=l;r()[b/$+ Q]=h;r()[b/$+ U]=g});b.wbg.__wbindgen_is_string=(a=>{const b=typeof c(a)===Z;return b});b.wbg.__wbindgen_is_object=(a=>{const b=c(a);const d=typeof b===`object`&&b!==P;return d});b.wbg.__wbindgen_is_undefined=(a=>{const b=c(a)===O;return b});b.wbg.__wbindgen_in=((a,b)=>{const d=c(a) in c(b);return d});b.wbg.__wbindgen_jsval_loose_eq=((a,b)=>{const d=c(a)==c(b);return d});b.wbg.__wbindgen_boolean_get=(a=>{const b=c(a);const d=typeof b===Y?(b?Q:U):2;return d});b.wbg.__wbindgen_number_get=((a,b)=>{const d=c(b);const e=typeof d===X?d:O;t()[a/8+ Q]=p(e)?U:e;r()[a/$+ U]=!p(e)});b.wbg.__wbg_getwithrefkey_5e6d9547403deab8=((a,b)=>{const d=c(a)[c(b)];return g(d)});b.wbg.__wbg_error_71d6845bf00a930f=((b,c)=>{var d=F(b,c).slice();a.__wbindgen_free(b,c*$,$);console.error(...d)});b.wbg.__wbg_body_3eb73da919b867a1=(a=>{const b=c(a).body;return p(b)?U:g(b)});b.wbg.__wbg_createElement_1a136faad4101f43=function(){return C(((a,b,d)=>{const e=c(a).createElement(k(b,d));return g(e)}),arguments)};b.wbg.__wbg_createElementNS_d47e0c50fa2904e0=function(){return C(((a,b,d,e,f)=>{const h=c(a).createElementNS(b===U?O:k(b,d),k(e,f));return g(h)}),arguments)};b.wbg.__wbg_createTextNode_dbdd908f92bae1b1=((a,b,d)=>{const e=c(a).createTextNode(k(b,d));return g(e)});b.wbg.__wbg_querySelector_d86f889797c65e88=function(){return C(((a,b,d)=>{const e=c(a).querySelector(k(b,d));return p(e)?U:g(e)}),arguments)};b.wbg.__wbg_instanceof_Window_99dc9805eaa2614b=(a=>{let b;try{b=c(a) instanceof Window}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_document_5257b70811e953c0=(a=>{const b=c(a).document;return p(b)?U:g(b)});b.wbg.__wbg_location_0f233324e8e8c699=(a=>{const b=c(a).location;return g(b)});b.wbg.__wbg_history_370f36be0803466b=function(){return C((a=>{const b=c(a).history;return g(b)}),arguments)};b.wbg.__wbg_instanceof_Element_f614cf57d4316979=(a=>{let b;try{b=c(a) instanceof Element}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_namespaceURI_0819c2800784a176=((b,d)=>{const e=c(d).namespaceURI;var f=p(e)?U:o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);var g=l;r()[b/$+ Q]=g;r()[b/$+ U]=f});b.wbg.__wbg_setinnerHTML_99deeacfff0ae4cc=((a,b,d)=>{c(a).innerHTML=k(b,d)});b.wbg.__wbg_outerHTML_69934f9195df65af=((b,d)=>{const e=c(d).outerHTML;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ Q]=g;r()[b/$+ U]=f});b.wbg.__wbg_children_3ab614807b5f0709=(a=>{const b=c(a).children;return g(b)});b.wbg.__wbg_removeAttribute_5c264e727b67dbdb=function(){return C(((a,b,d)=>{c(a).removeAttribute(k(b,d))}),arguments)};b.wbg.__wbg_setAttribute_0918ea45d5a1c663=function(){return C(((a,b,d,e,f)=>{c(a).setAttribute(k(b,d),k(e,f))}),arguments)};b.wbg.__wbg_href_1ab7f03b8a745310=function(){return C(((b,d)=>{const e=c(d).href;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ Q]=g;r()[b/$+ U]=f}),arguments)};b.wbg.__wbg_pathname_2cd8b46817926b06=function(){return C(((b,d)=>{const e=c(d).pathname;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ Q]=g;r()[b/$+ U]=f}),arguments)};b.wbg.__wbg_search_eb68df82d26f8761=function(){return C(((b,d)=>{const e=c(d).search;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ Q]=g;r()[b/$+ U]=f}),arguments)};b.wbg.__wbg_hash_9bd16c0f666cdf27=function(){return C(((b,d)=>{const e=c(d).hash;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ Q]=g;r()[b/$+ U]=f}),arguments)};b.wbg.__wbg_bubbles_f0783dc095f8e220=(a=>{const b=c(a).bubbles;return b});b.wbg.__wbg_cancelBubble_191799b8e0ab3254=(a=>{const b=c(a).cancelBubble;return b});b.wbg.__wbg_composedPath_d94a39b8c8f6eed1=(a=>{const b=c(a).composedPath();return g(b)});b.wbg.__wbg_addEventListener_1b158e9e95e0ab00=function(){return C(((a,b,d,e,f)=>{c(a).addEventListener(k(b,d),c(e),c(f))}),arguments)};b.wbg.__wbg_removeEventListener_177ff96081e6f22d=function(){return C(((a,b,d,e,f)=>{c(a).removeEventListener(k(b,d),c(e),f!==U)}),arguments)};b.wbg.__wbg_setchecked_3b12f3d602a63e47=((a,b)=>{c(a).checked=b!==U});b.wbg.__wbg_value_c93cb4b4d352228e=((b,d)=>{const e=c(d).value;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ Q]=g;r()[b/$+ U]=f});b.wbg.__wbg_setvalue_9bd3f93b3864ddbf=((a,b,d)=>{c(a).value=k(b,d)});b.wbg.__wbg_state_cabf8868613a7bdb=function(){return C((a=>{const b=c(a).state;return g(b)}),arguments)};b.wbg.__wbg_href_dc647488029294b4=((b,d)=>{const e=c(d).href;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ Q]=g;r()[b/$+ U]=f});b.wbg.__wbg_pathname_d0d5b2fd2c7d8243=((b,d)=>{const e=c(d).pathname;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ Q]=g;r()[b/$+ U]=f});b.wbg.__wbg_search_b5c7b044aaf64616=((b,d)=>{const e=c(d).search;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ Q]=g;r()[b/$+ U]=f});b.wbg.__wbg_hash_286eced2921b7b34=((b,d)=>{const e=c(d).hash;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ Q]=g;r()[b/$+ U]=f});b.wbg.__wbg_new_7d30e9d9d2deaf9d=function(){return C(((a,b)=>{const c=new URL(k(a,b));return g(c)}),arguments)};b.wbg.__wbg_newwithbase_1151109a3f062f92=function(){return C(((a,b,c,d)=>{const e=new URL(k(a,b),k(c,d));return g(e)}),arguments)};b.wbg.__wbg_parentNode_f3957fdd408a62f7=(a=>{const b=c(a).parentNode;return p(b)?U:g(b)});b.wbg.__wbg_parentElement_86a7612dde875ba9=(a=>{const b=c(a).parentElement;return p(b)?U:g(b)});b.wbg.__wbg_lastChild_8f7b6f3825115eff=(a=>{const b=c(a).lastChild;return p(b)?U:g(b)});b.wbg.__wbg_nextSibling_13e9454ef5323f1a=(a=>{const b=c(a).nextSibling;return p(b)?U:g(b)});b.wbg.__wbg_setnodeValue_8656e865e9b11bbb=((a,b,d)=>{c(a).nodeValue=b===U?O:k(b,d)});b.wbg.__wbg_textContent_efe8338af53ddf62=((b,d)=>{const e=c(d).textContent;var f=p(e)?U:o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);var g=l;r()[b/$+ Q]=g;r()[b/$+ U]=f});b.wbg.__wbg_appendChild_bd383ec5356c0bdb=function(){return C(((a,b)=>{const d=c(a).appendChild(c(b));return g(d)}),arguments)};b.wbg.__wbg_insertBefore_882082ef4c5d7766=function(){return C(((a,b,d)=>{const e=c(a).insertBefore(c(b),c(d));return g(e)}),arguments)};b.wbg.__wbg_removeChild_14b08321b677677a=function(){return C(((a,b)=>{const d=c(a).removeChild(c(b));return g(d)}),arguments)};b.wbg.__wbg_instanceof_ShadowRoot_cb6366cb0956ce29=(a=>{let b;try{b=c(a) instanceof ShadowRoot}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_host_99e27ed8897850f2=(a=>{const b=c(a).host;return g(b)});b.wbg.__wbg_value_ab23a75318ea828f=((b,d)=>{const e=c(d).value;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ Q]=g;r()[b/$+ U]=f});b.wbg.__wbg_setvalue_918a8ae77531a942=((a,b,d)=>{c(a).value=k(b,d)});b.wbg.__wbg_get_c43534c00f382c8a=((a,b)=>{const d=c(a)[b>>>U];return g(d)});b.wbg.__wbg_length_d99b680fd68bf71b=(a=>{const b=c(a).length;return b});b.wbg.__wbg_newnoargs_5859b6d41c6fe9f7=((a,b)=>{const c=new Function(k(a,b));return g(c)});b.wbg.__wbg_call_a79f1973a4f07d5e=function(){return C(((a,b)=>{const d=c(a).call(c(b));return g(d)}),arguments)};b.wbg.__wbg_new_87d841e70661f6e9=(()=>{const a=new a0();return g(a)});b.wbg.__wbg_self_086b5302bcafb962=function(){return C((()=>{const a=self.self;return g(a)}),arguments)};b.wbg.__wbg_window_132fa5d7546f1de5=function(){return C((()=>{const a=window.window;return g(a)}),arguments)};b.wbg.__wbg_globalThis_e5f801a37ad7d07b=function(){return C((()=>{const a=globalThis.globalThis;return g(a)}),arguments)};b.wbg.__wbg_global_f9a61fce4af6b7c1=function(){return C((()=>{const a=global.global;return g(a)}),arguments)};b.wbg.__wbg_from_a663e01d8dab8e44=(a=>{const b=M.from(c(a));return g(b)});b.wbg.__wbg_instanceof_ArrayBuffer_f4521cec1b99ee35=(a=>{let b;try{b=c(a) instanceof ArrayBuffer}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_isSafeInteger_d8c89788832a17bf=(a=>{const b=Number.isSafeInteger(c(a));return b});b.wbg.__wbg_getTime_af7ca51c0bcefa08=(a=>{const b=c(a).getTime();return b});b.wbg.__wbg_new0_c0e40662db0749ee=(()=>{const a=new Date();return g(a)});b.wbg.__wbg_entries_7a47f5716366056b=(a=>{const b=a0.entries(c(a));return g(b)});b.wbg.__wbg_is_a5728dbfb61c82cd=((a,b)=>{const d=a0.is(c(a),c(b));return d});b.wbg.__wbg_resolve_97ecd55ee839391b=(a=>{const b=Promise.resolve(c(a));return g(b)});b.wbg.__wbg_then_7aeb7c5f1536640f=((a,b)=>{const d=c(a).then(c(b));return g(d)});b.wbg.__wbg_buffer_5d1b598a01b41a42=(a=>{const b=c(a).buffer;return g(b)});b.wbg.__wbg_new_ace717933ad7117f=(a=>{const b=new V(c(a));return g(b)});b.wbg.__wbg_set_74906aa30864df5a=((a,b,d)=>{c(a).set(c(b),d>>>U)});b.wbg.__wbg_length_f0764416ba5bb237=(a=>{const b=c(a).length;return b});b.wbg.__wbg_instanceof_Uint8Array_4f5cffed7df34b2f=(a=>{let b;try{b=c(a) instanceof V}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_set_37a50e901587b477=function(){return C(((a,b,d)=>{const e=Reflect.set(c(a),c(b),c(d));return e}),arguments)};b.wbg.__wbindgen_debug_string=((b,d)=>{const e=u(c(d));const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ Q]=g;r()[b/$+ U]=f});b.wbg.__wbindgen_throw=((a,b)=>{throw new T(k(a,b))});b.wbg.__wbindgen_memory=(()=>{const b=a.memory;return g(b)});b.wbg.__wbindgen_closure_wrapper257=((a,b,c)=>{const d=v(a,b,85,w);return g(d)});b.wbg.__wbindgen_closure_wrapper744=((a,b,c)=>{const d=v(a,b,301,z);return g(d)});b.wbg.__wbindgen_closure_wrapper881=((a,b,c)=>{const d=v(a,b,354,A);return g(d)});b.wbg.__wbindgen_closure_wrapper904=((a,b,c)=>{const d=v(a,b,366,B);return g(d)});return b});var E=(()=>{if(D===P||D.byteLength===U){D=new Uint32Array(a.memory.buffer)};return D});var t=(()=>{if(s===P||s.byteLength===U){s=new Float64Array(a.memory.buffer)};return s});var B=((c,d,e)=>{try{a.wasm_bindgen__convert__closures__invoke1_mut_ref__h459d0af4579510cf(c,d,y(e))}finally{b[x++]=O}});var e=(a=>{if(a<132)return;b[a]=d;d=a});var v=((b,c,d,e)=>{const f={a:b,b:c,cnt:Q,dtor:d};const g=(...b)=>{f.cnt++;const c=f.a;f.a=U;try{return e(c,f.b,...b)}finally{if(--f.cnt===U){a.__wbindgen_export_2.get(f.dtor)(c,f.b)}else{f.a=c}}};g.original=f;return g});var o=((a,b,c)=>{if(c===O){const c=m.encode(a);const d=b(c.length,Q)>>>U;j().subarray(d,d+ c.length).set(c);l=c.length;return d};let d=a.length;let e=b(d,Q)>>>U;const f=j();let g=U;for(;g<d;g++){const b=a.charCodeAt(g);if(b>127)break;f[e+ g]=b};if(g!==d){if(g!==U){a=a.slice(g)};e=c(e,d,d=g+ a.length*3,Q)>>>U;const b=j().subarray(e+ g,e+ d);const f=n(a,b);g+=f.written};l=g;return e});var j=(()=>{if(i===P||i.byteLength===U){i=new V(a.memory.buffer)};return i});var k=((a,b)=>{a=a>>>U;return h.decode(j().subarray(a,a+ b))});var y=(a=>{if(x==Q)throw new T(`out of js stack`);b[--x]=a;return x});let a;const b=new M(N).fill(O);b.push(O,P,!0,!1);let d=b.length;const h=typeof TextDecoder!==R?new TextDecoder(S,{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw T(`TextDecoder not available`)}};if(typeof TextDecoder!==R){h.decode()};let i=P;let l=U;const m=typeof TextEncoder!==R?new TextEncoder(S):{encode:()=>{throw T(`TextEncoder not available`)}};const n=typeof m.encodeInto===W?((a,b)=>m.encodeInto(a,b)):((a,b)=>{const c=m.encode(a);b.set(c);return {read:a.length,written:c.length}});let q=P;let s=P;let x=N;let D=P;export default L;export{K as initSync}