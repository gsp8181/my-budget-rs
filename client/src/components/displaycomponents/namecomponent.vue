<template>
    <div class="field">
        <input type="text" v-on:change="$emit('on-save', process())" v-model="text"></input>
    </div>
</template>
<script>
    // :value="text"
    //TODO: do not mutate prop
    //http://cssdeck.com/labs/iicbd9ko
    export default {
        props: ['name', 'text', 'id'],
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

    /*TODO: FIX*/
    /*input:before {
        content: "£"
    }*/

    field {
        /*margin: 50px;
  padding: 5px;*/
        background: transparent;
        display: inline-block;
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
        width: 100%;
    }
</style>