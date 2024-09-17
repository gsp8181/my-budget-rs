<template>
    <div class="table-responsive">
        <table id="maintable" class="table table-dark table-hover table-striped table-sm">
            <thead>
                <tr>
                    <th v-for="object in objects">{{object.displayName}}</th>
                    <th></th>
                </tr>
            </thead>
            <tbody>
                <tr v-for="(element, index) in info">
                    <td v-for="object in objects" :data-id="element.id" :data-type="object.name" :tabindex="index"
                        :class="object.clazz">
                        <component :is="object.innerComponent" v-on:on-save="onSave" :name="object.name"
                            :id="element.id" :text="element[object.name]">
                        </component>
                    </td>
                    <td class="noedit">
                        <button type="button" class="close delete-data" :data-id="element.id" v-on:click="dele(element.id)" aria-label="Close">
                            <span aria-hidden="true">×</span></button>
                    </td>
                </tr>
            </tbody>
        </table>
    </div>
</template>
<script>
    //TODO: delete
    //TODO: rendering
    import axios from 'axios';
    import namecomponent from '@/components/displaycomponents/namecomponent.vue';
    import currencycomponent from '@/components/displaycomponents/currencycomponent.vue';
    import cardcomponent from '@/components/displaycomponents/cardcomponent.vue';
    import daycomponent from '@/components/displaycomponents/daycomponent.vue';
    export default {
        name: 'datatable',
        props: {
            api: {
                type: String,
                required: true
            },
            objects: {
                type: Array,
                required: true
            },
            refreshing: {
                type: Boolean,
                required: true
            }
        },
        //TODO: format currency
        components: {
            namecomponent,
            currencycomponent,
            cardcomponent,
            daycomponent
        },
        data() {
            return {
                info: null
            }
        },
        watch: {
            // call again the method if the route changes
            //TODO: callback watch
            '$route': 'fetchData',
            refreshing: function () {
                if (this.refreshing) {
                    this.fetchData();
                    this.$emit('refreshed');
                }
            }
        },
        // when the view is created this function will run.
        created: function () {
            this.fetchData()
        },
        methods: {
            fetchData() {
                axios
                    .get(this.api)
                    .then(response => (this.info = response.data)) //TODO: wat do fail?
            },
            onSave: function (dataObject) {
                var data = {}
                data[dataObject.name] = dataObject.text;
                axios.put(this.api + "/" + dataObject.id, data)
                    .then(function (response) {
                        //TODO: done? spinner vif?
                    })
                    .catch(function (error) {
                        //TODO:alert component
                        console.log(error);
                    });
            },
            dele: function (dataObject)
            {
                let self = this;
                //TODO: change to name
                //TODO: jazz up to html5 bootstrap
                //TODO: put object name
               if(confirm("Are you sure you wish to delete whatever with id: " + (dataObject)))
               { 
                   axios.delete(this.api + "/" + dataObject)
                    .then(function (response) {
                        self.fetchData();
                    })
                    .catch(function (error) {
                        //TODO:alert component
                        //TODO: console.log error 
                        console.log(error);
                    });
}
            }
        }
    }
</script>