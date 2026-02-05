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
const quickToggleInput = document.querySelector(".quick-toggle-input");
const quickToggleLabel = document.querySelector(".quick-toggle-label");
const stepButtons = document.querySelectorAll(".step-pill");
const installerStages = document.querySelectorAll(".installer-stage");
const installerNext = document.querySelector(".installer-next");
const installerPrev = document.querySelector(".installer-prev");

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
    setInstallerStep("1");
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

if (quickToggleInput && quickToggleLabel) {
  const updateQuickLabel = () => {
    quickToggleLabel.textContent = quickToggleInput.checked ? "Включено" : "Выключено";
  };
  quickToggleInput.addEventListener("change", updateQuickLabel);
  updateQuickLabel();
}

const setInstallerStep = (step) => {
  stepButtons.forEach((button) => {
    const isActive = button.dataset.step === step;
    button.classList.toggle("active", isActive);
  });
  installerStages.forEach((stage) => {
    stage.classList.toggle("hidden", stage.dataset.step !== step);
  });
};

stepButtons.forEach((button) => {
  button.addEventListener("click", () => {
    setInstallerStep(button.dataset.step);
  });
});

if (installerNext && installerPrev) {
  installerNext.addEventListener("click", () => {
    const active = document.querySelector(".step-pill.active");
    const current = Number.parseInt(active?.dataset.step ?? "1", 10);
    const next = Math.min(current + 1, 3);
    setInstallerStep(String(next));
  });

  installerPrev.addEventListener("click", () => {
    const active = document.querySelector(".step-pill.active");
    const current = Number.parseInt(active?.dataset.step ?? "1", 10);
    const prev = Math.max(current - 1, 1);
    setInstallerStep(String(prev));
  });
}
