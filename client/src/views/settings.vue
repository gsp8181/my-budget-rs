<template>
    <div>
        <h1 class="h3 mb-0 text-gray-800">Settings</h1>

        <form ref="settingsForm" v-on:submit.prevent="onSubmit">
            <div class="form-group">
                <label for="payday">Payday</label>
                <input type="number" class="form-control" min="1" max="31" id="payday" name="payday"
                    aria-describedby="paydayHelp" placeholder="Enter payday" v-model="info.payday">
                <small id="paydayHelp" class="form-text text-muted">The day of the month you are paid monthly.</small>
            </div>
            <div class="form-group">
                <label for="weekdaySaving">Weekday Saving</label>
                <input type="number" class="form-control" step="0.01" id="weekdaySaving" name="weekdaySaving"
                    aria-describedby="weekdaySavingHelp" v-model="info.weekdaySaving" placeholder="Enter weekdaySaving">
                <small id="weekdaySavingHelp" class="form-text text-muted">The amount you wish to save Monday-Thursday
                    for the weekend.</small>
            </div>
            <div class="form-group">
                <label for="dailyRate">Daily Rate</label>
                <input type="number" class="form-control" step="0.01" id="dailyRate" name="dailyRate"
                    aria-describedby="dailyRateHelp" v-model="info.dailyRate" placeholder="Enter dailyRate">
                <small id="dailyRateHelp" class="form-text text-muted">The total amount you wish to be made available
                    per day.</small>
            </div>
            <div class="form-group">
                <label for="pay">Total Pay</label>
                <input type="number" class="form-control" step="0.01" id="pay" v-model="info.pay" name="pay"
                    aria-describedby="payHelp" placeholder="Enter pay">
                <small id="payHelp" class="form-text text-muted">The total net amount you get paid each month.</small>
            </div>
            <button type="submit" class="btn btn-primary">Submit</button>
        </form>
    </div>
</template>
<script>
    //TODO: settings menu disappears when on settings
    import axios from 'axios';
    export default {
        name: 'settings',
        data() {
            return {
                info: {
                    payday: 0,
                    weekdaySaving: 0,
                    dailyRate: 0,
                    pay: 0
                }
            }
        },
        created: function () {
            this.fetchData()
        },
        methods: {
            fetchData() {
                let self = this;
                axios
                    .get('api/settings')
                    .then(function (response) {
                        for (var key in response.data) {
                            var element = response.data[key];
                            self.info[element['name']] = element['value'];
                        }
                    })
            },
            onSubmit(event) {
                axios.post('api/settings', this.info)
                    .then(function (response) {
                        //TODO: done? spinner vif?
                        this.refresh();
                    })
                    .catch(function (error) {
                        //TODO:alert component
                        console.log(error);
                    });
            },
        },
    }
</script>