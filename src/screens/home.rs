use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    // get settings
    // show week summary
    // upcomming vacations
    view! {
        <section class="landing-page">
            <div class="hero stack">
                <h1 class="title">"Track Time, Not Paperwork"</h1>
                <p class="tagline">"The simplest way to clock in and out"</p>
                <a href="/login" class="cta-button">
                    "Get Started Free"
                </a>
            </div>

            <div class="features stack">
                <div class="feature-card">
                    <i name="clock"></i>
                    <h3>"One-Click Clock In"</h3>
                    <p>"No more paper cards or complex systems. Just tap and go."</p>
                </div>

                <div class="feature-card">
                    <i name="calendar"></i>
                    <h3>"Smart Scheduling"</h3>
                    <p>"Track hours, breaks, and overtime with a click."</p>
                </div>

                <div class="feature-card">
                    <i name="chart"></i>
                    <h3>"Real-Time Reports"</h3>
                    <p>"Instant insights into attendance and work hours."</p>
                </div>
            </div>

            <div class="social-proof stack">
                <h2>"Trusted by Growing Teams"</h2>
                <div class="testimonials">
                    <blockquote>
                        <p>
                            "Clkr simplified our entire time tracking process. Our team loves it!"
                        </p>
                        <cite>"- Sarah M., Small Business Owner"</cite>
                    </blockquote>
                </div>
            </div>

            <div class="cta-section stack">
                <h2>"Ready to Simplify Time Tracking?"</h2>
                <p>"Join thousands of happy teams using Clkr"</p>
                <a href="/login" class="cta-button">
                    "Start Your Free Trial"
                </a>
            </div>
        </section>
    }
}
