<template>
  <!-- Page Wrapper -->
  <div id="wrapper">

    <!-- Sidebar -->
    <ul class="navbar-nav bg-gradient-primary sidebar sidebar-dark accordion" id="accordionSidebar">

      <!-- Sidebar - Brand -->
      <a class="sidebar-brand d-flex align-items-center justify-content-center" href="index.php">
        <div class="sidebar-brand-icon rotate-n-15">
          <!--<i class="fas fa-laugh-wink"></i>-->
          <img src="./assets/charge-for-access.png" width="39" height="39" />
        </div>
        <div class="sidebar-brand-text mx-3">Budgeting <sup>2</sup></div>
      </a>

      <!-- Divider -->
      <hr class="sidebar-divider my-0">

      <!-- Nav Item - Dashboard -->
      <li class="nav-item ">
        <router-link class="nav-link" to="/">
          <i class="fas fa-fw fa-tachometer-alt"></i>
          <span>Dashboard</span></router-link>
      </li>

      <!-- Divider -->
      <hr class="sidebar-divider">

      <!-- Heading -->
      <div class="sidebar-heading">
        Credit
      </div>

      <!-- Nav Item - Tables -->
      <li class="nav-item ">
        <router-link class="nav-link" to="/bank">
          <i class="fas fa-fw fa-university"></i>
          <span>Bank</span></router-link>
      </li>

      <!-- Nav Item - Tables -->
      <li class="nav-item ">
        <router-link class="nav-link" to="/cash">
          <i class="fas fa-fw fa-money-bill"></i>
          <span>Cash</span></router-link>
      </li>

      <!-- Nav Item - Tables -->
      <li class="nav-item ">
        <router-link class="nav-link" to="/regularcredit">
          <i class="fas fa-fw fa-table"></i>
          <span>Regular Credit</span></router-link>
      </li>

      <!-- Nav Item - Tables -->
      <li class="nav-item ">
        <router-link class="nav-link" to="/cardheld">
          <i class="fas fa-fw fa-table"></i>
          <span>Card Items Held Off Balance</span></router-link>
      </li>

      <!-- Nav Item - Tables -->
      <li class="nav-item ">
        <router-link class="nav-link" to="/uncleared">
          <i class="fas fa-fw fa-spinner"></i>
          <span>Uncleared Item</span></router-link>
      </li>

      <!-- Nav Item - Tables -->
      <li class="nav-item ">
        <router-link class="nav-link" to="/debt">
          <i class="fas fa-fw fa-table"></i>
          <span>Debt Owed To Me</span></router-link>
      </li>

      <!-- Nav Item - Tables -->
      <li class="nav-item ">
        <router-link class="nav-link" to="/misccredit">
          <i class="fas fa-fw fa-table"></i>
          <span>Misc Credit</span></router-link>
      </li>

      <!-- Divider -->
      <hr class="sidebar-divider">

      <!-- Heading -->
      <div class="sidebar-heading">
        Debit
      </div>

      <!-- Nav Item - Tables -->
      <li class="nav-item ">
        <router-link class="nav-link" to="/cardbalance">
          <i class="fas fa-fw fa-credit-card"></i>
          <span>Card Balance</span></router-link>
      </li>

      <!-- Nav Item - Tables -->
      <li class="nav-item ">
        <router-link class="nav-link" to="/regularpayment">
          <i class="fas fa-fw fa-table"></i>
          <span>Regular Payment</span></router-link>
      </li>

      <!-- Nav Item - Tables -->
      <li class="nav-item ">
        <router-link class="nav-link" to="/debtto">
          <i class="fas fa-fw fa-table"></i>
          <span>Debt I Owe</span></router-link>
      </li>

      <!-- Nav Item - Tables -->
      <li class="nav-item ">
        <router-link class="nav-link" to="/miscdebit">
          <i class="fas fa-fw fa-table"></i>
          <span>Misc Debit</span></router-link>
      </li>

      <!-- Divider -->
      <hr class="sidebar-divider d-none d-md-block">

      <!-- Sidebar Toggler (Sidebar) -->
      <div class="text-center d-none d-md-inline">
        <button class="rounded-circle border-0" id="sidebarToggle"></button>
      </div>

    </ul>
    <!-- End of Sidebar -->

    <!-- Content Wrapper -->
    <div id="content-wrapper" class="d-flex flex-column">

      <!-- Main Content -->
      <div id="content">

        <!-- Topbar -->
        <nav class="navbar navbar-expand navbar-light bg-white topbar mb-4 static-top shadow">

          <!-- Sidebar Toggle (Topbar) -->
          <button id="sidebarToggleTop" class="btn btn-link d-md-none rounded-circle mr-3">
            <i class="fa fa-bars"></i>
          </button>

          <!-- Topbar Navbar -->
          <ul class="navbar-nav ml-auto">

            <div class="topbar-divider d-none d-sm-block"></div>

            <!-- Nav Item - User Information -->
            <li class="nav-item dropdown no-arrow">
              <a class="nav-link dropdown-toggle" href="#" id="userDropdown" role="button" data-toggle="dropdown"
                aria-haspopup="true" aria-expanded="false">
                <span class="mr-2 d-none d-lg-inline text-gray-600 small">username</span>
                <i class="fas fa-user-circle img-profile fa-2x"></i>
                <!--<img class="img-profile rounded-circle" src="https://source.unsplash.com/QAB-WJcbgJk/60x60">-->
              </a>
              <!-- Dropdown - User Information -->
              <div class="dropdown-menu dropdown-menu-right shadow animated--grow-in" aria-labelledby="userDropdown">
                <!--TODO: DISAPPEARS WHEN ON SETTINGS-->
                  <router-link class="dropdown-item" to="/settings">
                  <i class="fas fa-cogs fa-sm fa-fw mr-2 text-gray-400"></i>
                  Settings
                  </router-link>
                <div class="dropdown-divider"></div>
                <a class="dropdown-item" href="#" data-toggle="modal" data-target="#logoutModal">
                  <i class="fas fa-sign-out-alt fa-sm fa-fw mr-2 text-gray-400"></i>
                  Logout
                </a>
              </div>
            </li>

          </ul>

        </nav>
        <!-- End of Topbar -->

        <!-- Begin Page Content -->
        <div class="container-fluid">

          <router-view></router-view>
        </div>
        <!-- End of Main Content -->

        <!-- Footer -->
        <footer class="sticky-footer bg-white">
          <div class="container my-auto">
            <div class="copyright text-center my-auto">
              <span>Copyright &copy; Geoffrey Prytherch 2018-2019</span>
            </div>
          </div>
        </footer>
        <!-- End of Footer -->

      </div>
      <!-- End of Content Wrapper -->

    </div>
    <!-- End of Page Wrapper -->

    <!-- Scroll to Top Button-->
    <a class="scroll-to-top rounded" href="#page-top">
      <i class="fas fa-angle-up"></i>
    </a>

    <!-- Logout Modal-->
    <div class="modal fade" id="logoutModal" tabindex="-1" role="dialog" aria-labelledby="exampleModalLabel"
      aria-hidden="true">
      <div class="modal-dialog" role="document">
        <div class="modal-content">
          <div class="modal-header">
            <h5 class="modal-title" id="exampleModalLabel">Ready to Leave?</h5>
            <button class="close" type="button" data-dismiss="modal" aria-label="Close">
              <span aria-hidden="true">×</span>
            </button>
          </div>
          <div class="modal-body">Select "Logout" below if you are ready to end your current session.</div>
          <div class="modal-footer">
            <button class="btn btn-secondary" type="button" data-dismiss="modal">Cancel</button>
            <a class="btn btn-primary" href="login.html">Logout</a>
          </div>
        </div>
      </div>
    </div>

  </div>
</template>
<script>
  import $ from 'jquery'
  //TODO: import styles from './@fortawesome/fontawesome-free/css/all.min.css'
  export default {
    mounted: function () {
      this.loadfeatures()
    },
    methods: {
      loadfeatures: function () {
        // Toggle the side navigation
        $("#sidebarToggle, #sidebarToggleTop").on('click', function (e) {
          $("body").toggleClass("sidebar-toggled");
          $(".sidebar").toggleClass("toggled");
          if ($(".sidebar").hasClass("toggled")) {
            $('.sidebar .collapse').collapse('hide');
          };
        });

        // Close any open menu accordions when window is resized below 768px
        $(window).resize(function () {
          if ($(window).width() < 768) {
            $('.sidebar .collapse').collapse('hide');
          };
        });

        // Prevent the content wrapper from scrolling when the fixed side navigation hovered over
        $('body.fixed-nav .sidebar').on('mousewheel DOMMouseScroll wheel', function (e) {
          if ($(window).width() > 768) {
            var e0 = e.originalEvent,
              delta = e0.wheelDelta || -e0.detail;
            this.scrollTop += (delta < 0 ? 1 : -1) * 30;
            e.preventDefault();
          }
        });

        // Scroll to top button appear
        $(document).on('scroll', function () {
          var scrollDistance = $(this).scrollTop();
          if (scrollDistance > 100) {
            $('.scroll-to-top').fadeIn();
          } else {
            $('.scroll-to-top').fadeOut();
          }
        });

        // Smooth scrolling using jQuery easing
        $(document).on('click', 'a.scroll-to-top', function (e) {
          var $anchor = $(this);
          $('html, body').stop().animate({
            scrollTop: ($($anchor.attr('href')).offset().top)
          }, 1000, 'easeInOutExpo');
          e.preventDefault();
        });        
        
        //import 'startbootstrap-sb-admin-2/js/sb-admin-2.min.js'
      }
    },
  }
</script>
