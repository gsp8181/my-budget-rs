(window["webpackJsonp"]=window["webpackJsonp"]||[]).push([["owedto"],{a639:function(e,a,t){"use strict";t.r(a);var n=function(){var e=this,a=e._self._c;return a("div",[a("div",{attrs:{id:"contentPage"}},[a("pageheading",{attrs:{name:"Debtee"},on:{"call-refresh":e.refreshChild}},[e._v("Debt Owed")]),a("popupadd",{attrs:{data:e.data,name:"Debt",api:"api/debtto"},on:{"call-refresh":e.refreshChild},scopedSlots:e._u([{key:"default",fn:function(t){return[a("namepopup",{attrs:{name:"Debtee",placeholder:"Owed for Pizza"},model:{value:t.data.name,callback:function(a){e.$set(t.data,"name",a)},expression:"slotProps.data.name"}}),a("amountpopup",{model:{value:t.data.amount,callback:function(a){e.$set(t.data,"amount",a)},expression:"slotProps.data.amount"}})]}}])}),a("datatable",{attrs:{refreshing:this.refresh,api:"api/debtto",objects:[{name:"name",displayName:"Debtee",innerComponent:"namecomponent"},{name:"amount",displayName:"Amount",innerComponent:"currencycomponent"}]},on:{refreshed:e.childRefreshComplete}})],1)])},o=[],r=t("1cdd"),d=t("f7a1"),s=t("fc11"),p=t("d8bc"),l=t("bf16"),i={components:{popupadd:r["a"],pageheading:d["a"],amountpopup:s["a"],namepopup:p["a"],datatable:l["a"]},data(){return{data:{name:"",amount:""},refresh:!1}},methods:{refreshChild:function(){this.refresh=!0},childRefreshComplete:function(){this.refresh=!1}}},u=i,m=t("2877"),c=Object(m["a"])(u,n,o,!1,null,null,null);a["default"]=c.exports}}]);
//# sourceMappingURL=owedto.23225a93.js.map