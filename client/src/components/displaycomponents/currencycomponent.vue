<template>
    <div class="field">
        <span><input type="number" step="0.01" v-on:change="$emit('on-save', process())"
                onchange="(function(el){el.value=parseFloat(el.value).toFixed(2);})(this)"
                v-model="text"></input></span>
    </div>
</template>
<script>
    //TODO: do not mutate prop
    //TODO: generic
    //TODO: validation
    //TODO: test onchange
    export default {
        props: ['name', 'text', 'id'],
        created: function () {
            this.text = parseFloat(this.text).toFixed(2);
        },
        methods: {
            process() {
                return {
                    name: this.name,
                    text: this.text,
                    id: this.id
                };
            }
        }
    }
</script>
<style scoped>
    ::placeholder {
        color: #fff;
    }

    span:before {
        content: "£";
        position: relative;
        margin-right: -20px
    }

    span {
        width: auto;
    }

    field {
        background: transparent;
        /*display: inline-block;*/
    }


    input,
    select,
    textarea {
        color: #fff;
    }

    textarea:focus,
    input:focus {
        color: #fff;
    }

    input {
        border: none;
        background: transparent;
        outline: none;
        padding-left: 20px;
        width: 100%;
    }
</style>