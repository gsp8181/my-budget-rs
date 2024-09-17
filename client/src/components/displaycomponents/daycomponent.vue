<template>
    <div class="field">
        <span><input type="number" step="1" min="1" max="31" v-on:change="$emit('on-save', process())" v-model="text"></input></span>
    </div>
</template>
<script>
    //TODO: do not mutate prop
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

    span {
        width: auto;
    }

    field {
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