<template>
    <div>
        <select v-model="selectedcd" data-populate="cardbalance" name="cardid" id="bankinput-card"
            class="form-control form-control-sm inputmodal-elem populate-select" @change="onChange($event)">
            <option v-for="(element) in info" :value="element.id" :key="element.id">{{element.name}}</option>
        </select>
    </div>
</template>
<script>
    //this.$emit('eventName', data) to update

    //TODO: v-model
    //TODO: remove old handling code?


    import axios from 'axios';
    export default {
        props: {
            selectedcard: {
                //TODO: parse string
                type: Number,
                default: function () {
                    return -1;
                },
                value: Number
            },
        },
        data() {
            return {
                info: null,
                selectedcd: -1
            }
        },
        watch: {
            // call again the method if the route changes
            //'$route': 'fetchData'
            //TODO: card updated?
        },
        // when the view is created this function will run.
        created: function () {
            this.fetchData();
            if (this.selectedcard != -1) {
                this.selectedcd = this.selectedcard;
            }
        },
        methods: {
            fetchData() {
                var self = this;
                axios //TODO: error
                    .get('api/cardbalance')
                    .then(function (response) {
                        self.info = response.data;
                        if (self.selectedcard == -1) {
                            self.selectedcd = self.info[0].id;
                            self.$emit('selected-change', {
                                new: self.selectedcd
                            });
                            //TODO: not new
                            //TODO: vmodel
                        }
                    })
                                        .catch(function (error) {
                        //TODO:alert component
                        console.log(error);
                    });
            },
            onChange(event) {
                this.$emit('selected-change', {
                    new: event.target.value
                });
                //$emit('input', $event.target.value);
            }
        }
    }
</script>