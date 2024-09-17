<template>
    <div>
        <div id="contentPage">
            <pageheading v-on:call-refresh="refreshChild" name="Card Balance">Card Balance</pageheading>

            <popupadd :data="data" v-on:call-refresh="refreshChild" name="Card Balance" api="api/cardbalance">
                <template v-slot:default="slotProps">
                    <namepopup v-model="slotProps.data.name" name="Card Name" placeholder="Main Credit Card">
                    </namepopup>
                    <amountpopup v-model="slotProps.data.amount"></amountpopup>
                </template>
            </popupadd>

            <datatable :refreshing="this.refresh" v-on:refreshed="childRefreshComplete" api="api/cardbalance" v-bind:objects="[{name: 'name', displayName:'Card Name', innerComponent:'namecomponent'},
                 {name: 'amount', displayName:'Amount', innerComponent:'currencycomponent'}]">
            </datatable>
        </div>
    </div>
</template>
<script>
    //v-model="data.name"
    import popupadd from '@/components/popup-add.vue';
    import pageheading from '@/components/page-heading.vue';
    import amountpopup from '@/components/inputs/amount-popup.vue';
    import namepopup from '@/components/inputs/name-popup.vue';
    import datatable from '@/components/datatable.vue';
    export default {
        components: {
            popupadd,
            pageheading,
            amountpopup,
            namepopup,
            datatable
        },



        //TODO: make a slot
        data() {
            return {
                data: {
                    name: "",
                    amount: ""
                },
                refresh: false,
            }
        },
        methods: {
            refreshChild: function () {
                this.refresh = true;
            },
            childRefreshComplete: function () {
                this.refresh = false;
            }
        }
    }
</script>