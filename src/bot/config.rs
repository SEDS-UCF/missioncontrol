use serenity::model::id::{ChannelId, GuildId, RoleId};

pub const SEND_INTRO: bool = false;

pub const GUILD_ID: GuildId = GuildId(491275273598402561);

pub const CAT_CHANNELS: ChannelId = ChannelId(614536824295260160);

pub const CAT_GAMES: ChannelId = ChannelId(696569774632861746);

pub const EXCLUDED_CHANNELS: &[ChannelId] = &[
	ChannelId(669328124357640222), // Hydrazine
];

pub const MEMBERSHIP_MEMBER: RoleId = RoleId(585637350529302529);
pub const MEMBERSHIP_ALUMNI: RoleId = RoleId(612059569274748969);
pub const MEMBERSHIP_FRIEND: RoleId = RoleId(787427932346777660);

pub const ALLOWED_MEMBERSHIPS: &[RoleId] = &[
	MEMBERSHIP_MEMBER, // SEDS Member
	MEMBERSHIP_ALUMNI, // SEDS Alumnus
	MEMBERSHIP_FRIEND, // Friend of SEDS
];

pub const ALLOWED_ROLES: &[RoleId] = &[
	RoleId(621586486793601044), // Industry Pro
	RoleId(709650648421105694), // Industry Intern
	RoleId(759187648799178785), // Student Researcher
];

pub const ALLOWED_PROJECTS: &[RoleId] = &[
	RoleId(787477836171968552), // RASC-AL
	RoleId(585634734122467339), // IREC
	RoleId(787478051414867978), // Sojourner
	RoleId(787478212644962305), // Liquid Bi-Prop
	RoleId(787478390705487922), // FSGC Hybrids
];

pub const MAX_LIST_SIZE: usize = 20;