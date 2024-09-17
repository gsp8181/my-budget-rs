<template>
    <div>
        <div id="contentPage">
            <pageheading v-on:call-refresh="refreshChild" name="Uncleared Payment">Uncleared Payment on Card</pageheading>

            <popupadd :data="data" v-on:call-refresh="refreshChild" name="Uncleared Payment" api="api/uncleared">
                <template v-slot:default="slotProps">
                    <namepopup v-model="slotProps.data.name" name="Item" placeholder="Offline Card Payment"></namepopup>
                    <amountpopup v-model="slotProps.data.amount"></amountpopup>
                    <cardusedpopup v-model="slotProps.data.cardid"></cardusedpopup>
                </template>
            </popupadd>

            <datatable :refreshing="this.refresh" v-on:refreshed="childRefreshComplete" api="api/uncleared"
                v-bind:objects="[{name: 'name', displayName:'Item', innerComponent:'namecomponent'},
                 {name: 'amount', displayName:'Amount', innerComponent:'currencycomponent'},
                 {name: 'cardid', displayName:'Card Used', innerComponent:'cardcomponent'}]">
            </datatable>
        </div>
    </div>
</template>




<script>
    import popupadd from '@/components/popup-add.vue';
    import pageheading from '@/components/page-heading.vue';
    import amountpopup from '@/components/inputs/amount-popup.vue';
    import cardusedpopup from '@/components/inputs/cardused-popup.vue';
    import namepopup from '@/components/inputs/name-popup.vue';
    import datatable from '@/components/datatable.vue';
    export default {
        components: {
            popupadd,
            pageheading,
            amountpopup,
            cardusedpopup,
            namepopup,
            datatable
        },


        //TODO: card select MUST give valid option in API
        //TODO: make a slot
        //TODO: date taken
        data() {
            return {
                data: {
                    name: "",
                    amount: "",
                    cardid: -1,
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