(window["webpackJsonp"]=window["webpackJsonp"]||[]).push([["debt"],{"76ca":function(e,a,t){"use strict";t.r(a);t("7f7f");var n=function(){var e=this,a=e._self._c;return a("div",[a("div",{attrs:{id:"contentPage"}},[a("pageheading",{attrs:{name:"Debt"},on:{"call-refresh":e.refreshChild}},[e._v("Debt")]),a("popupadd",{attrs:{data:e.data,name:"Debt",api:"api/debt"},on:{"call-refresh":e.refreshChild},scopedSlots:e._u([{key:"default",fn:function(t){return[a("namepopup",{attrs:{name:"Debtor",placeholder:"John Doe"},model:{value:t.data.name,callback:function(a){e.$set(t.data,"name",a)},expression:"slotProps.data.name"}}),a("amountpopup",{model:{value:t.data.amount,callback:function(a){e.$set(t.data,"amount",a)},expression:"slotProps.data.amount"}})]}}])}),a("datatable",{attrs:{refreshing:this.refresh,api:"api/debt",objects:[{name:"name",displayName:"Debtor",innerComponent:"namecomponent"},{name:"amount",displayName:"Amount",innerComponent:"currencycomponent"}]},on:{refreshed:e.childRefreshComplete}})],1)])},o=[],r=t("1cdd"),s=t("f7a1"),p=t("fc11"),d=t("d8bc"),l=t("bf16"),i={components:{popupadd:r["a"],pageheading:s["a"],amountpopup:p["a"],namepopup:d["a"],datatable:l["a"]},data:function(){return{data:{name:"",amount:""},refresh:!1}},methods:{refreshChild:function(){this.refresh=!0},childRefreshComplete:function(){this.refresh=!1}}},u=i,c=t("2877"),m=Object(c["a"])(u,n,o,!1,null,null,null);a["default"]=m.exports}}]);
//# sourceMappingURL=debt.9908a392.js.map