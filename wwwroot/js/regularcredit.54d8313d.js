(window["webpackJsonp"]=window["webpackJsonp"]||[]).push([["regularcredit"],{"770f":function(a,e,t){"use strict";t.r(e);var n=function(){var a=this,e=a._self._c;return e("div",[e("div",{attrs:{id:"contentPage"}},[e("pageheading",{attrs:{name:"Creditor"},on:{"call-refresh":a.refreshChild}},[a._v("Regular Credit")]),e("popupadd",{attrs:{data:a.data,name:"Regular Credit",api:"api/regularcredit"},on:{"call-refresh":a.refreshChild},scopedSlots:a._u([{key:"default",fn:function(t){return[e("namepopup",{attrs:{name:"Creditor",placeholder:"Rental Income"},model:{value:t.data.name,callback:function(e){a.$set(t.data,"name",e)},expression:"slotProps.data.name"}}),e("amountpopup",{model:{value:t.data.amount,callback:function(e){a.$set(t.data,"amount",e)},expression:"slotProps.data.amount"}}),e("daypopup",{model:{value:t.data.day,callback:function(e){a.$set(t.data,"day",e)},expression:"slotProps.data.day"}})]}}])}),e("datatable",{attrs:{refreshing:this.refresh,api:"api/regularcredit",objects:[{name:"name",displayName:"Creditor",innerComponent:"namecomponent"},{name:"amount",displayName:"Amount",innerComponent:"currencycomponent"},{name:"day",displayName:"Day Taken",innerComponent:"daycomponent"}]},on:{refreshed:a.childRefreshComplete}})],1)])},r=[],o=t("1cdd"),s=t("f7a1"),l=t("fc11"),p=t("d8bc"),d=t("bf16"),u=t("ca4d"),i={components:{popupadd:o["a"],pageheading:s["a"],amountpopup:l["a"],namepopup:p["a"],datatable:d["a"],daypopup:u["a"]},data(){return{data:{name:"",amount:"",day:-1},refresh:!1}},methods:{refreshChild:function(){this.refresh=!0},childRefreshComplete:function(){this.refresh=!1}}},c=i,m=t("2877"),f=Object(m["a"])(c,n,r,!1,null,null,null);e["default"]=f.exports},ca4d:function(a,e,t){"use strict";var n=function(){var a=this,e=a._self._c;return e("div",{staticClass:"form-group"},[e("label",{staticClass:"col-form-label",attrs:{for:"message-text"}},[a._v("Day Taken:")]),e("div",{staticClass:"input-group"},[e("input",{staticClass:"form-control inputmodal-elem",attrs:{type:"number",placeholder:"25",step:"1",min:"1",max:"31",name:"day",required:""},domProps:{value:a.value},on:{input:function(e){return a.$emit("input",e.target.value)}}})])])},r=[],o={name:"day-popup",props:["value"]},s=o,l=t("2877"),p=Object(l["a"])(s,n,r,!1,null,null,null);e["a"]=p.exports}}]);
//# sourceMappingURL=regularcredit.54d8313d.js.map