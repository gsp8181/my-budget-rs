<template>
<!--TODO:ADD IN ACTIVE-->
  <div id="azz">
    <div>
      <!-- Page Heading -->
      <div class="d-sm-flex align-items-center justify-content-between mb-4">
        <h1 class="h3 mb-0 text-gray-800">Dashboard</h1>
      </div>

      <!--<h2>Balance Held</h2>-->

      <!-- Earnings (Monthly) Card Example -->
      <div class="row">
        <div class="col-xl-3 col-md-6 mb-4">
          <div class="card border-left-primary shadow h-100 py-2">
            <div class="card-body">
              <div class="row no-gutters align-items-center">
                <div class="col mr-2">
                  <div class="text-xs font-weight-bold text-primary text-uppercase mb-1">Balance Held</div>
                  <div id="data-balance" class="h5 mb-0 font-weight-bold text-gray-800">£{{info.amount}}</div>
                </div>
                <div class="col-auto">
                  <i class="fas fa-calendar fa-2x text-gray-300"></i>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
      <div class="row">
        <!-- Earnings (Monthly) Card Example -->
        <div class="col-xl-3 col-md-6 mb-4">
          <div class="card border-left-success shadow h-100 py-2">
            <div class="card-body">
              <div class="row no-gutters align-items-center">
                <div class="col mr-2">
                  <div class="text-xs font-weight-bold text-success text-uppercase mb-1">End Of Week Total</div>
                  <div id="data-end_of_week" class="h5 mb-0 font-weight-bold text-gray-800">£{{info.end_of_week}}</div>
                </div>
                <div class="col-auto">
                  <i class="fas fa-pound-sign fa-2x text-gray-300"></i>
                </div>
              </div>
            </div>
          </div>
        </div>

        <div class="col-xl-3 col-md-6 mb-4">
          <div class="card border-left-success shadow h-100 py-2">
            <div class="card-body">
              <div class="row no-gutters align-items-center">
                <div class="col mr-2">
                  <div class="text-xs font-weight-bold text-success text-uppercase mb-1">Full Weekend Total</div>
                  <div id="data-full_weekend" class="h5 mb-0 font-weight-bold text-gray-800">£{{info.full_weekend}}</div>
                </div>
                <div class="col-auto">
                  <i class="fas fa-pound-sign fa-2x text-gray-300"></i>
                </div>
              </div>
            </div>
          </div>
        </div>

        <div class="col-xl-3 col-md-6 mb-4">
          <div class="card border-left-success shadow h-100 py-2">
            <div class="card-body">
              <div class="row no-gutters align-items-center">
                <div class="col mr-2">
                  <div class="text-xs font-weight-bold text-success text-uppercase mb-1">Remaining Week Total</div>
                  <div id="data-remaining_week" class="h5 mb-0 font-weight-bold text-gray-800">£{{info.remaining_week}}</div>
                </div>
                <div class="col-auto">
                  <i class="fas fa-pound-sign fa-2x text-gray-300"></i>
                </div>
              </div>
            </div>
          </div>
        </div>


      </div>
      <div class="row">

        <!-- Content Column -->
        <div class="col-lg-10 mb-10">

          <!-- Project Card Example -->
          <div class="card shadow mb-4">
            <div class="card-header py-3">
              <h6 class="m-0 font-weight-bold text-primary">Payments Today</h6>
            </div>
            <div class="card-body">
              <table id="paymentsTable" class="table">
                <thead>
                  <tr>
                    <th scope="col">#</th>
                    <th scope="col">Name</th>
                    <th scope="col">Amount</th>
                  </tr>
                </thead>
                <tbody>
                  <tr v-for="(element, index) in info.today">
                    <td scope='row'>{{ index + 1 }}</td>
                    <td>{{ element.name }}</td>
                    <td>{{ element.amount }}</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>


        </div>
      </div>

      <div class="row">
        <div class="col-xl-3 col-md-6 mb-4">
          <div class="card border-left-success shadow h-100 py-2">
            <div class="card-body">
              <div class="row no-gutters align-items-center">
                <div class="col mr-2">
                  <div class="text-xs font-weight-bold text-success text-uppercase mb-1">Total Monthly Credits</div>
                  <div id="data-monthly_credits" class="h5 mb-0 font-weight-bold text-gray-800">£{{info.monthly_credits}}</div>
                </div>
                <div class="col-auto">
                  <i class="fas fa-pound-sign fa-2x text-gray-300"></i>
                </div>
              </div>
            </div>
          </div>
        </div>
        <!-- Earnings (Monthly) Card Example -->
        <div class="col-xl-3 col-md-6 mb-4">
          <div class="card border-left-success shadow h-100 py-2">
            <div class="card-body">
              <div class="row no-gutters align-items-center">
                <div class="col mr-2">
                  <div class="text-xs font-weight-bold text-success text-uppercase mb-1">Total Monthly Debits</div>
                  <div id="data-monthly_debits" class="h5 mb-0 font-weight-bold text-gray-800">£{{info.monthly_debits}}</div>
                </div>
                <div class="col-auto">
                  <i class="fas fa-pound-sign fa-2x text-gray-300"></i>
                </div>
              </div>
            </div>
          </div>
        </div>



        <div class="col-xl-3 col-md-6 mb-4">
          <div class="card border-left-success shadow h-100 py-2">
            <div class="card-body">
              <div class="row no-gutters align-items-center">
                <div class="col mr-2">
                  <div class="text-xs font-weight-bold text-success text-uppercase mb-1">Card Balance Held</div>
                  <div id="data-card_held" class="h5 mb-0 font-weight-bold text-gray-800">£{{info.card_held_total}}</div>
                </div>
                <div class="col-auto">
                  <i class="fas fa-pound-sign fa-2x text-gray-300"></i>
                </div>
              </div>
            </div>
          </div>
        </div>


      </div>

      <div class="row">
        <div class="col-xl-3 col-md-6 mb-4">
          <div class="card border-left-success shadow h-100 py-2">
            <div class="card-body">
              <div class="row no-gutters align-items-center">
                <div class="col mr-2">
                  <div class="text-xs font-weight-bold text-success text-uppercase mb-1">Monthly Saved</div>
                  <div id="data-net_saved_avg" class="h5 mb-0 font-weight-bold text-gray-800">£{{info.net_saved_avg}}</div>
                </div>
                <div class="col-auto">
                  <i class="fas fa-pound-sign fa-2x text-gray-300"></i>
                </div>
              </div>
            </div>
          </div>
        </div>

        <div class="col-xl-3 col-md-6 mb-4">
          <div class="card border-left-success shadow h-100 py-2">
            <div class="card-body">
              <div class="row no-gutters align-items-center">
                <div class="col mr-2">
                  <div class="text-xs font-weight-bold text-success text-uppercase mb-1">Yearly Saved</div>
                  <div id="data-saved_this_year" class="h5 mb-0 font-weight-bold text-gray-800">£{{info.saved_this_year}}</div>
                </div>
                <div class="col-auto">
                  <i class="fas fa-pound-sign fa-2x text-gray-300"></i>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
//TODO: go to top isn't working
  import axios from 'axios';
  export default {
    //module.exports = {
    data() {
      return {
        info: null
      }
    },
    watch: {
      // call again the method if the route changes
      '$route': 'fetchData'
    },
    // when the view is created this function will run.
    created: function () {
      this.fetchData()
    },
    methods: {
      fetchData() {
        axios
          .get('api/')
          .then(response => (this.info = response.data))
      }
    }
  }
</script>