<script>
  import { enhance } from "$app/forms";

  // @ts-nocheck
  import { onMount } from "svelte";

  onMount(() => {
    // document.addEventListener("DOMContentLoaded", function () {
    //   var areaOfInterest = document.getElementsByName("sponsor_interest")[0];
    //   var amountField = document.querySelector(
    //     '.input-field.col-lg-12 textarea[name="sponsor_message"]'
    //   );
    //   var button = document.querySelector(
    //     ".input-field.col-lg-12 button.qu_btn"
    //   );
    //   console.log(areaOfInterest);
    //   areaOfInterest.addEventListener("change", function () {
    //     console.log(areaOfInterest.value);
    //     if (areaOfInterest.value === "Donation") {
    //       amountField.setAttribute("placeholder", "Enter Donation Amount");
    //       amountField.setAttribute("type", "number");
    //       button.textContent = "Donate Now";
    //       console.log("donation");
    //     } else {
    //       amountField.setAttribute(
    //         "placeholder",
    //         "Tell Us How You Would Like to Contribute..."
    //       );
    //       amountField.setAttribute("type", "text");
    //       button.textContent = "Connect with Us";
    //       console.log("connect");
    //     }
    //   });
    // });
  });
</script>

<div class="container largeContainer">
  <div class="row">
    <div class="col-xl-7">
      <div class="appointment_form">
        <p>Become a Sponsor</p>
        <h3>Explore Sponsorship Opportunities</h3>
        <form
          action="http://localhost/auth/login"
          method="post"
          class="row"
          id="sponsorship_form"
          use:enhance
        >
          <div class="input-field col-lg-6">
            <input
              class="required"
              type="text"
              name="password"
              placeholder="Password"
            />
          </div>
          <div class="input-field col-lg-6">
            <input
              class="required"
              type="email"
              name="sponsor_email"
              placeholder="Email Address"
            />
          </div>
          <div class="input-field icRight col-lg-12 select-area">
            <select
              class="required"
              name="sponsor_interest"
              style="display: none;"
            >
              <option selected>Area of Interest</option>
              <option>Project Funding</option>
              <option>Donation</option>
              <option>Sponsorship</option>
            </select>
            <div class="nice-select required">
              <span class="current">Area of Interest</span>
              <ul class="list">
                <li data-value="Area of Interest" class="option selected">
                  Project Funding
                </li>
                <li data-value="Sponsorship Opportunities" class="option">
                  Donation
                </li>
                <li data-value="Donation Options" class="option">
                  Sponsorship
                </li>
              </ul>
            </div>
          </div>
          <div class="input-field col-lg-6">
            <i class="twi-calendar2"></i>
            <input
              class="required"
              type="text"
              name="sponsor_date"
              placeholder="Date of Interest"
            />
          </div>
          <div class="input-field col-lg-6">
            <i class="twi-clock2"></i>
            <input
              class="required"
              type="text"
              name="sponsor_time"
              placeholder="Time of Interest"
            />
          </div>
          <div class="input-field col-lg-12">
            <textarea
              class="required"
              name="sponsor_message"
              placeholder="Tell Us How You Would Like to Contribute..."
            ></textarea>
          </div>
          <div class="input-field col-lg-12">
            <button type="submit" class="qu_btn">Connect with Us</button>
            <div class="sponsorship_message"></div>
          </div>
        </form>
      </div>
    </div>
    <div class="col-xl-5">
      <div class="reviewArea">
        <h2 class="secTitle">Support Our Cause</h2>
        <p class="secDesc">
          Your contribution makes a difference in our mission to empower and
          uplift. Join us in creating a positive impact on our community.
        </p>
        <div class="cusRating">
          <i class="twi-star"></i><i class="twi-star"></i><i class="twi-star"
          ></i><i class="twi-star"></i><i class="twi-star"></i>
        </div>
        <p class="customers">
          <span>Thank You to Our Generous Donors</span> for their support towards
          our initiatives.
        </p>
        <div class="single_skill" data-parcent="70">
          <p>Projects Funded</p>
          <div class="ss_parent">
            <div
              class="ss_child"
              style="width: 70.0635%; overflow: hidden;"
            ></div>
            <span style="left: 69.9996%;">70%</span>
          </div>
        </div>
        <div class="single_skill" data-parcent="80">
          <p>Donor Satisfaction</p>
          <div class="ss_parent">
            <div
              class="ss_child"
              style="width: 80.1011%; overflow: hidden;"
            ></div>
            <span style="left: 79.9991%;">80%</span>
          </div>
        </div>
        <div class="hpAuthor">
          <img class="author" src="/images/home3/author.png" alt="" />
          <img width="100" src="/images/home3/sign_light.webp" alt="" />
          <strong>
            <div class="">Rev. Dinna Osayi – <span>Founder</span></div>
          </strong>
        </div>
      </div>
    </div>
  </div>
</div>
