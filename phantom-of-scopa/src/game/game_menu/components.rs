use bevy::prelude::Component;

#[derive(Component, Debug)]
pub struct InGameMenuUI;

#[derive(Component, Debug)]
pub struct InGameMenuRootNode;

#[derive(Component, Debug)]
pub struct RootInGameMenuUI;

#[derive(Component, Debug)]
pub struct SettingsUi;

#[derive(Component, Debug)]
pub struct VolumeSettingsButton(pub usize);

#[derive(Component, Debug)]
pub struct SettingsButton;

#[derive(Component, Debug)]
pub struct SelectedVolume;

#[derive(Component, Debug)]
pub struct MainMenuButton;

#[derive(Component, Debug)]
pub struct BackToRootButton;
