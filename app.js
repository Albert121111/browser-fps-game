const tabs = document.querySelectorAll(".tab");
const panels = document.querySelectorAll("[data-panel]");
const profileLabel = document.querySelector(".current-profile");
const switcherChips = document.querySelectorAll(".switcher-chip");
const downloadButtons = document.querySelectorAll(".download-toggle");
const installerModal = document.querySelector(".installer-modal");
const installerOpenButtons = document.querySelectorAll(".installer-open");
const installerCloseButtons = document.querySelectorAll(".installer-close");
const memorySliders = document.querySelectorAll(".memory-slider");
const memoryValues = document.querySelectorAll(".memory-value");

const showPanel = (target) => {
  panels.forEach((panel) => {
    const isMatch = panel.dataset.panel === target;
    panel.classList.toggle("hidden", !isMatch);
  });

  tabs.forEach((tab) => {
    const isActive = tab.dataset.tab === target;
    tab.classList.toggle("active", isActive);
    tab.setAttribute("aria-selected", String(isActive));
  });
};

tabs.forEach((tab) => {
  tab.addEventListener("click", () => showPanel(tab.dataset.tab));
});

switcherChips.forEach((chip) => {
  chip.addEventListener("click", () => {
    switcherChips.forEach((button) => button.classList.remove("active"));
    chip.classList.add("active");
    if (profileLabel) {
      profileLabel.textContent = chip.dataset.profile ?? chip.textContent;
    }
  });
});

downloadButtons.forEach((button) => {
  button.addEventListener("click", () => {
    const nextState = button.dataset.action === "pause" ? "resume" : "pause";
    button.dataset.action = nextState;
    button.textContent = nextState === "pause" ? "Пауза" : "Продолжить";
  });
});

const progressNodes = document.querySelectorAll(".progress");
const tickProgress = () => {
  progressNodes.forEach((node) => {
    const current = Number.parseInt(node.dataset.progress ?? "0", 10);
    const next = Math.min(current + 1, 100);
    node.dataset.progress = String(next);
    node.textContent = `${next}%`;
  });
};

setInterval(tickProgress, 5000);

const toggleInstaller = (shouldOpen) => {
  if (!installerModal) {
    return;
  }
  installerModal.classList.toggle("hidden", !shouldOpen);
};

installerOpenButtons.forEach((button) => {
  button.addEventListener("click", () => {
    showPanel("installer");
    toggleInstaller(true);
  });
});

installerCloseButtons.forEach((button) => {
  button.addEventListener("click", () => {
    toggleInstaller(false);
    showPanel("preview");
  });
});

memorySliders.forEach((slider) => {
  slider.addEventListener("input", () => {
    memoryValues.forEach((valueNode) => {
      valueNode.textContent = slider.value;
    });
  });
});
