use leptos::*;

#[component]
pub fn Countdown(#[prop(into)] date: String) -> impl IntoView {
    return view! {
        <div id="countdown" class="text-accent text-center text-2xl font-fancy shadow-accent" />
        <script>
        let target = {format!("\"{}\"", date)};
        var countDownDate = new Date(target).getTime();

        var x = setInterval(function() {
            var now = new Date().getTime();
            var distance = countDownDate - now;

            var days = Math.floor(distance / (1000 * 60 * 60 * 24));
            var hours = Math.floor((distance % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60));
            var minutes = Math.floor((distance % (1000 * 60 * 60)) / (1000 * 60));
            var seconds = Math.floor((distance % (1000 * 60)) / 1000);

            if (distance > 0) {
                document.getElementById("countdown").innerHTML = String(days) + "d " + String(hours).padStart(2, "0") + ":" + String(minutes).padStart(2, "0") + ":" + String(seconds).padStart(2, "0");
            } else {
                clearInterval(x);
                document.getElementById("demo").innerHTML = "NOW";
            }
        }, 1000);
        </script>
    };
}
