<template>
    <div>
        <div id="contentPage">
            <pageheading v-on:call-refresh="refreshChild" name="Creditor">Regular Credit</pageheading>

            <popupadd :data="data" v-on:call-refresh="refreshChild" name="Regular Credit" api="api/regularcredit">
                <template v-slot:default="slotProps">
                    <namepopup v-model="slotProps.data.name" name="Creditor" placeholder="Rental Income">
                    </namepopup>
                    <amountpopup v-model="slotProps.data.amount"></amountpopup>
                    <daypopup v-model="slotProps.data.day"></daypopup>
                </template>
            </popupadd>

            <datatable :refreshing="this.refresh" v-on:refreshed="childRefreshComplete" api="api/regularcredit" v-bind:objects="[{name: 'name', displayName:'Creditor', innerComponent:'namecomponent'},
                 {name: 'amount', displayName:'Amount', innerComponent:'currencycomponent'},                 
                 {name: 'day', displayName: 'Day Taken', innerComponent:'daycomponent'}]">
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
    import daypopup from '@/components/inputs/day-popup.vue';
    export default {
        components: {
            popupadd,
            pageheading,
            amountpopup,
            namepopup,
            datatable,
            daypopup
        },

//TODO: date

        //TODO: make a slot
        data() {
            return {
                data: {
                    name: "",
                    amount: "",
                    day: -1
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