(window["webpackJsonp"]=window["webpackJsonp"]||[]).push([["bank"],{fc7b:function(a,e,n){"use strict";n.r(e);var t=function(){var a=this,e=a._self._c;return e("div",[e("div",{attrs:{id:"contentPage"}},[e("pageheading",{attrs:{name:"Bank Accounts"},on:{"call-refresh":a.refreshChild}},[a._v("Bank Accounts")]),e("popupadd",{attrs:{data:a.data,name:"Bank Account",api:"api/bank"},on:{"call-refresh":a.refreshChild},scopedSlots:a._u([{key:"default",fn:function(n){return[e("namepopup",{attrs:{name:"Account",placeholder:"Main Current Account"},model:{value:n.data.name,callback:function(e){a.$set(n.data,"name",e)},expression:"slotProps.data.name"}}),e("amountpopup",{model:{value:n.data.amount,callback:function(e){a.$set(n.data,"amount",e)},expression:"slotProps.data.amount"}})]}}])}),e("datatable",{attrs:{refreshing:this.refresh,api:"api/bank",objects:[{name:"name",displayName:"Account",innerComponent:"namecomponent"},{name:"amount",displayName:"Amount",innerComponent:"currencycomponent"}]},on:{refreshed:a.childRefreshComplete}})],1)])},o=[],r=n("1cdd"),s=n("f7a1"),c=n("fc11"),p=n("d8bc"),u=n("bf16"),d={components:{popupadd:r["a"],pageheading:s["a"],amountpopup:c["a"],namepopup:p["a"],datatable:u["a"]},data(){return{data:{name:"",amount:""},refresh:!1}},methods:{refreshChild:function(){this.refresh=!0},childRefreshComplete:function(){this.refresh=!1}}},l=d,i=n("2877"),m=Object(i["a"])(l,t,o,!1,null,null,null);e["default"]=m.exports}}]);
//# sourceMappingURL=bank.72d72613.js.map