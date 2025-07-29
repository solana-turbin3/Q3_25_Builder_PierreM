import cliConfetti from "cli-confetti";
import CliUpdate from "cli-update";

const SECONDS = 1000;

cliConfetti({}, function (error, confetti) {
  if (error) throw error;
  CliUpdate.render(confetti);

  // Stop confetti after 5 seconds
  setTimeout(() => {
    CliUpdate.render("🎉 Congratulations on passing all the tests! 🎉");
    process.exit(0);
  }, 5 * SECONDS);
});
