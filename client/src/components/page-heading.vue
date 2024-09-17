<template>
    <div class="d-sm-flex align-items-center justify-content-between mb-4">
        <h1 class="h3 mb-0 text-gray-800"><slot></slot></h1>
        <div class="btn-group mr-2">
            <a href="#" class="d-sm-inline-block btn btn-sm btn-primary shadow-sm" data-toggle="modal" data-target="#addModal"><i
                    class="fas fa-plus-circle fa-sm text-white-50"></i> Add</a>
            <a href="#" v-on:click="$emit('call-refresh')" class="d-sm-inline-block btn btn-sm btn-primary shadow-sm refreshbutton"><i class="fas fa-sync fa-sm text-white-50"></i>
                Refresh</a>
        </div>
    </div>
</template>
<script>
//TODO: refresh emit fetchdata
//TODO: remove name
    export default {
        name: 'bank-heading',
        props: [],
        data() {
            return {}
        }
    }
</script>