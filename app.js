const tabs = document.querySelectorAll(".tab");
const panels = document.querySelectorAll("[data-panel]");
const profileLabel = document.querySelector(".current-profile");
const switcherChips = document.querySelectorAll(".switcher-chip");
const downloadButtons = document.querySelectorAll(".download-toggle");
const navLinks = document.querySelectorAll(".nav-link");
const installerOpenButtons = document.querySelectorAll(".installer-open");
const installerCloseButtons = document.querySelectorAll(".installer-close");
const memorySliders = document.querySelectorAll(".memory-slider");
const memoryValues = document.querySelectorAll(".memory-value");
const quickToggleInput = document.querySelector(".quick-toggle-input");
const quickToggleLabel = document.querySelector(".quick-toggle-label");
const actionToast = document.querySelector(".action-toast");
const buttons = document.querySelectorAll("button");

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

navLinks.forEach((link) => {
  link.addEventListener("click", (event) => {
    event.preventDefault();
    const target = link.dataset.panel;
    if (target) {
      showPanel(target);
      navLinks.forEach((item) => item.classList.remove("active"));
      link.classList.add("active");
    }
  });
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

installerOpenButtons.forEach((button) => {
  button.addEventListener("click", () => {
    showPanel("installer");
  });
});

installerCloseButtons.forEach((button) => {
  button.addEventListener("click", () => {
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

const updateToast = (message) => {
  if (!actionToast) {
    return;
  }
  actionToast.textContent = message;
};

buttons.forEach((button) => {
  button.addEventListener("click", () => {
    const label = button.dataset.actionLabel || button.textContent?.trim() || "Готово";
    updateToast(`Действие: ${label}`);

    const target = button.dataset.panelTarget;
    if (target) {
      showPanel(target);
    }
  });
});
