<template>
    <div ref="vuemodal" class="modal fade" id="addModal" tabindex="-1" role="dialog" aria-labelledby="addModalLabel"
        aria-hidden="true">
        <div class="modal-dialog" role="document">
            <div class="modal-content">
                <div class="modal-header">
                    <h5 class="modal-title" id="addModalLabel">New {{name}}</h5>
                    <button type="button" class="close" data-dismiss="modal" aria-label="Close">
                        <span aria-hidden="true">&times;</span>
                    </button>
                </div>
                <form ref="addform" v-on:submit.prevent="onSubmit">
                    <div class="modal-body" ref="modalform">
                        <slot v-bind:data="data"></slot>
                    </div>
                    <div class="modal-footer">
                        <button type="button" class="btn btn-secondary" data-dismiss="modal">Close</button>
                        <button type="submit" class="btn btn-primary">Add {{name}}</button>
                    </div>
                </form>
            </div>
        </div>
    </div>
</template>
<script>
    //v-on:hidden.bs.modal.native="modalHidden" 
    //todo: actually bind directly to event
    import axios from 'axios';
    export default {
        name: 'popup-add',
        props: ['name', 'api', 'data'],
        methods: {
            modalHidden() {
this.$refs.addform.reset();
                //TODO: hide
                //TODO: clear
            },
            refresh()
            {
                this.$emit('call-refresh');
            },
            onSubmit(event) {
                $(this.$refs.vuemodal).modal('toggle');
                //TODO: NO, use scoped slots
                let self = this;

                var form = this.$refs.modalform;
                var jsonData = JSON.stringify(this.data);

                //var data = {}
                //data[dataObject.name] = dataObject.text;
                axios.post(this.api, this.data)
                    .then(function (response) {
                        //TODO: done? spinner vif?
                        self.refresh();
                    })
                    .catch(function (error) {
                        //TODO:alert component
                        //TODO: handle error 200 when param missing
                        //TODO: param 0
                        console.log(error);
                    });
            },
        },
        mounted() {
            $(this.$refs.vuemodal).on("hidden.bs.modal", this.modalHidden);
        },


    }
    //TODO: error handling
</script>