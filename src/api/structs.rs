use serde::{Deserialize, Serialize};

// These are the structs and API endpoints for the Lemmy API.
// By re-creating these from their HTTP API documentation, I was able to create the necessary tools for interacting with the Lemmy API, without needing to resort to using their Rust crate and thus be subjected to the AGPLv3.
// There are likely bugs in these structs, as I have already encountered params that were apparently optional when they were not marked as such. If you encounter such an issue, please let me know, or edit the struct param that is causing the issue.
// Most isslues likely can be easily fixed by adding the #[serde(skip_serializing_if = "Option::is_none")] attribute above the paramater in question, and then wrapping the param type with Option<>.
// This struct will eventually get spun off into it's own crate, that is better documented, to allow others to more easily interact with the Lemmy API, in a wider variety of projects than the could be allowed by the current Rust crate's license.

// The endpoints for the Lemmy HTTP API, as found from here: https://join-lemmy.org/api/classes/LemmyHttp.html

pub mod api_endpoints {
    pub struct GetEndpoint(pub String);

    impl GetEndpoint {
        pub const GET_BANNED_PERSONS: &str = "user/banned";
        pub const GET_CAPTCHA: &str = "user/get_captcha";
        pub const GET_COMMENT: &str = "comment";
        pub const GET_COMMENTS: &str = "comment/list";
        pub const GET_COMMUNITY: &str = "community";
        pub const GET_FEDERATED_INSTANCES: &str = "federated_instances";
        pub const GET_MODLOG: &str = "modlog";
        pub const GET_PERSON_DETAILS: &str = "user";
        pub const GET_PERSON_MENTIONS: &str = "user/mention";
        pub const GET_POST: &str = "post";
        pub const GET_POSTS: &str = "post/list";
        pub const GET_PRIVATE_MESSAGES: &str = "private_message/list";
        pub const GET_REPLIES: &str = "user/replies";
        pub const GET_REPORT_COUNT: &str = "user/report_count";
        pub const GET_SITE: &str = "site";
        pub const GET_SITE_METADATA: &str = "post/site_metadata";
        pub const GET_UNREAD_COUNT: &str = "user/unread_count";
        pub const GET_UNREAD_REGISTRATION_APPLICATION_COUNT: &str =
            "admin/registration_application/count";
        pub const LIST_COMMENT_REPORTS: &str = "comment/report/list";
        pub const LIST_COMMUNITIES: &str = "community/list";
        pub const LIST_POST_REPORTS: &str = "post/report/list";
        pub const LIST_PRIVATE_MESSAGE_REPORTS: &str = "private_message/report/list";
        pub const LIST_REGISTRATION_APPLICATIONS: &str = "admin/registration_application/list";
        pub const RESOLVE_OBJECT: &str = "resolve_object";
        pub const SEARCH: &str = "search";
    }

    pub struct PostEndpoint(pub String);

    impl PostEndpoint {
        pub const ADD_ADMIN: &str = "admin/add";
        pub const ADD_MOD_TO_COMMUNITY: &str = "community/mod";
        pub const BAN_FROM_COMMUNITY: &str = "community/ban_user";
        pub const BAN_PERSON: &str = "user/ban";
        pub const BLOCK_COMMUNITY: &str = "community/block";
        pub const BLOCK_PERSON: &str = "user/block";
        pub const CREATE_COMMENT: &str = "comment";
        pub const CREATE_COMMENT_REPORT: &str = "comment/report";
        pub const CREATE_COMMUNITY: &str = "community";
        pub const CREATE_CUSTOM_EMOJI: &str = "custom_emoji";
        pub const CREATE_POST: &str = "post";
        pub const CREATE_POST_REPORT: &str = "post/report";
        pub const CREATE_PRIVATE_MESSAGE: &str = "private_message";
        pub const CREATE_PRIVATE_MESSAGE_REPORT: &str = "private_message/report";
        pub const CREATE_SITE: &str = "site";
        pub const DELETE_ACCOUNT: &str = "user/delete_account";
        pub const DELETE_COMMENT: &str = "comment/delete";
        pub const DELETE_COMMUNITY: &str = "community/delete";
        pub const DELETE_CUSTOM_EMOJI: &str = "custom_emoji/delete";
        pub const DELETE_POST: &str = "post/delete";
        pub const DELETE_PRIVATE_MESSAGE: &str = "private_message/delete";
        pub const DISTINGUISH_COMMENT: &str = "comment/distinguish";
        pub const FEATURE_POST: &str = "post/feature";
        pub const FOLLOW_COMMUNITY: &str = "community/follow";
        pub const LEAVE_ADMIN: &str = "user/leave_admin";
        pub const LIKE_COMMENT: &str = "comment/like";
        pub const LIKE_POST: &str = "post/like";
        pub const LOCK_POST: &str = "post/lock";
        pub const LOGIN: &str = "user/login";
        pub const MARK_ALL_AS_READ: &str = "user/mark_all_as_read";
        pub const MARK_COMMENT_REPLY_AS_READ: &str = "comment/mark_as_read";
        pub const MARK_PERSON_MENTION_AS_READ: &str = "user/mention/mark_as_read";
        pub const MARK_POST_AS_READ: &str = "post/mark_as_read";
        pub const MARK_PRIVATE_MESSAGE_AS_READ: &str = "private_message/mark_as_read";
        pub const PASSWORD_CHANGE_AFTER_RESET: &str = "user/password_change";
        pub const PASSWORD_RESET: &str = "user/password_reset";
        pub const PURGE_COMMENT: &str = "admin/purge/comment";
        pub const PURGE_COMMUNITY: &str = "admin/purge/community";
        pub const PURGE_PERSON: &str = "admin/purge/person";
        pub const PURGE_POST: &str = "admin/purge/post";
        pub const REGISTER: &str = "user/register";
        pub const REMOVE_COMMENT: &str = "comment/remove";
        pub const REMOVE_COMMUNITY: &str = "community/remove";
        pub const REMOVE_POST: &str = "post/remove";
        pub const TRANSFER_COMMUNITY: &str = "community/transfer";
        pub const UPLOAD_IMAGE: &str = "TBD?";
        pub const VERIFY_EMAIL: &str = "user/verify_email";
    }

    pub struct PutEndpoint(pub String);

    impl PutEndpoint {
        pub const APPROVE_REGISTRATION_APPLICATION: &str = "admin/registration_application/approve";
        pub const CHANGE_PASSWORD: &str = "user/change_password";
        pub const EDIT_COMMENT: &str = "comment";
        pub const EDIT_COMMUNITY: &str = "community";
        pub const EDIT_CUSTOM_EMOJI: &str = "custom_emoji";
        pub const EDIT_POST: &str = "post";
        pub const EDIT_PRIVATE_MESSAGE: &str = "private_message";
        pub const EDIT_SITE: &str = "site";
        pub const RESOLVE_COMMENT_REPORT: &str = "comment/report/resolve";
        pub const RESOLVE_POST_REPORT: &str = "post/report/resolve";
        pub const RESOLVE_PRIVATE_MESSAGE_REPORT: &str = "private_message/report/resolve";
        pub const SAVE_COMMENT: &str = "comment/save";
        pub const SAVE_POST: &str = "post/save";
        pub const SAVE_USER_SETTINGS: &str = "user/save_user_settings";
    }
}

pub mod router_endpoints {
    pub struct RouterEndpoint(pub String);

    impl RouterEndpoint {
        pub const COMMUNITY: &str = "community";
        pub const HOME: &str = "home";
        pub const USER: &str = "user";
    }
}

// The struct for the url constructor
#[derive(Serialize, Clone, Default)]
pub struct ApiUrlConstructor {
    pub endpoint: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<&'static str>,
}

// The ErrorResponse struct for the api request functions
#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

// The data structs for the Lemmy HTTP API, created from the methods found here: https://join-lemmy.org/api/index.html

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct AddAdmin {
    pub added: bool,
    pub auth: String,
    pub person_id: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct AddAdminResponse {
    pub admins: PersonView,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct AddModToCommunity {
    pub added: bool,
    pub auth: String,
    pub community_id: i32,
    pub person_id: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct AddModToCommunityResponse {
    pub community: CommunityModeratorView,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct AdminPurgeComment {
    pub admin_person_id: i32,
    pub id: i32,
    pub post_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub when_: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct AdminPurgeCommentView {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin: Option<Person>,
    pub admin_purge_comment: AdminPurgeComment,
    pub post: Post,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct AdminPurgeCommunity {
    admin_person_id: i32,
    id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<String>,
    pub when_: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct AdminPurgeCommunityView {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin: Option<Person>,
    pub admin_purge_community: AdminPurgeCommunity,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct AdminPurgePerson {
    pub admin_person_id: i32,
    pub id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub when_: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct AdminPurgePersonView {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin: Option<Person>,
    pub admin_purge_person: AdminPurgePerson,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct AdminPurgePost {
    pub admin_person_id: i32,
    pub community_id: i32,
    pub id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub when_: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct AdminPurgePostView {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin: Option<Person>,
    pub admin_purge_post: AdminPurgePost,
    pub community: Community,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ApproveRegistrationApplication {
    pub approve: bool,
    pub auth: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deny_reason: Option<String>,
    pub id: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct BanFromCommunity {
    pub auth: String,
    pub ban: bool,
    pub community_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<i32>,
    pub person_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_data: Option<bool>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct BanFromCommunityResponse {
    pub banned: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_view: Option<PersonView>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct BanPerson {
    pub auth: String,
    pub ban: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<i32>,
    pub person_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_data: Option<bool>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct BanPersonResponse {
    pub banned: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_view: Option<PersonView>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct BannedPersonsResponse {
    pub banned: PersonView,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct BlockCommunity {
    pub auth: String,
    pub block: bool,
    pub community_id: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct BlockCommunityResponse {
    pub blocked: bool,
    pub community: CommunityView,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct BlockPerson {
    pub auth: String,
    pub block: bool,
    pub person_id: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct BlockPersonResponse {
    pub blocked: bool,
    pub person: PersonView,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct CaptchaResponse {
    pub png: String,
    pub uuid: String,
    pub wav: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ChangePassword {
    pub auth: String,
    pub new_password: String,
    pub new_password_verify: String,
    pub old_password: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct Comment {
    pub ap_id: String,
    pub content: String,
    pub creator_id: i32,
    pub deleted: bool,
    pub distinguished: bool,
    pub id: i32,
    pub language_id: i32,
    pub local: bool,
    pub path: String,
    pub post_id: i32,
    pub published: String,
    pub removed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct CommentAggregates {
    pub child_count: i32,
    pub downvotes: i32,
    pub hot_rank: i32,
    pub id: i32,
    pub published: String,
    pub score: i32,
    pub upvotes: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct CommentReply {
    pub community_id: i32,
    pub id: i32,
    pub published: String,
    pub read: bool,
    pub receipient_id: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct CommentReplyResponse {
    pub comment_reply_view: CommentReplyView,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct CommentReplyView {
    pub comment: Comment,
    pub comment_reply: CommentReply,
    pub community: Community,
    pub counts: CommentAggregates,
    pub creator: Person,
    pub creator_banned_from_community: bool,
    pub creator_blocked: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my_vote: Option<i32>,
    pub post: Post,
    pub receipient: Person,
    pub saved: bool,
    pub subscribed: SubscribedType,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct CommentReport {
    pub comment_id: i32,
    pub creator_id: i32,
    pub id: i32,
    pub original_commnment_text: String,
    pub published: String,
    pub reason: String,
    pub resolved: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct CommentReportResponse {
    pub comment_report_view: CommentReportView,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct CommentReportView {
    pub comment: Comment,
    pub comment_creator: Person,
    pub comment_report: CommentReport,
    pub community: Community,
    pub counts: CommentAggregates,
    pub creator: Person,
    pub creator_banned_from_community: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my_vote: Option<i32>,
    pub post: Post,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver: Option<Person>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct CommentResponse {
    pub comment_view: CommentView,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_id: Option<String>,
    pub receipient_ids: Vec<i32>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct CommentView {
    pub comment: Comment,
    pub community: Community,
    pub counts: CommentAggregates,
    pub creator: Person,
    pub creator_banned_from_community: bool,
    pub creator_blocked: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my_vote: Option<i32>,
    pub post: Post,
    pub saved: bool,
    pub subscribed: SubscribedType,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct Community {
    pub actor_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,
    pub deleted: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers_url: Option<String>,
    pub hidden: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    pub id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_url: Option<String>,
    pub instance_id: i32,
    pub local: bool,
    pub name: String,
    pub nsfw: bool,
    pub posting_restricted_to_mods: bool,
    pub published: String,
    pub removed: bool,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct CommunityAggregates {
    pub comments: i32,
    pub community_id: i32,
    pub hot_rank: i32,
    pub id: i32,
    pub posts: i32,
    pub published: String,
    pub subscribers: i32,
    pub users_active_day: i32,
    pub users_active_half_year: i32,
    pub users_active_month: i32,
    pub users_active_week: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct CommunityBlockView {
    pub community: Community,
    pub person: Person,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct CommunityFollowerView {
    pub community: Community,
    pub person: Person,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct CommunityJoin {
    pub community_id: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct CommunityJoinResponse {
    joined: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]

pub struct CommunityModeratorView {
    pub community: Community,
    pub moderator: Person,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct CommunityResponse {
    pub community_view: CommunityView,
    pub discussion_languages: Vec<i32>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct CommunityView {
    pub blocked: bool,
    pub community: Community,
    pub counts: CommunityAggregates,
    pub subscribed: SubscribedType,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct CreateComment {
    pub auth: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<i32>,
    pub post_id: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct CreateCommentLike {
    pub auth: String,
    pub comment_id: i32,
    pub score: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct CreateCommentReport {
    pub auth: String,
    pub comment_id: i32,
    pub reason: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct CreateCommunity {
    pub auth: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discussion_languages: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posting_restricted_to_mods: Option<bool>,
    pub title: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct CreateCustomEmoji {
    pub alt_text: String,
    pub auth: String,
    pub category: String,
    pub image_url: String,
    pub keywords: Vec<String>,
    pub shortcode: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct CreatePost {
    pub auth: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    pub community_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honeypot: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_id: Option<i32>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct CreatePostLike {
    pub auth: String,
    pub post_id: i32,
    pub score: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct CreatePostReport {
    pub auth: String,
    pub post_id: i32,
    pub reason: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct CreatePrivateMessage {
    pub auth: String,
    pub content: String,
    pub receipient_id: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct CreatePrivateMessageReport {
    pub auth: String,
    pub private_message_id: i32,
    pub reason: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct CreateSite {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_name_max_length: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_instances: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_email_admins: Option<bool>,
    pub auth: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_instances: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha_difficulty: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub community_creation_admin_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_post_listing_type: Option<ListingType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_theme: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discussion_languages: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_downvotes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_nsfw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub federation_debug: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub federation_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_modlog_mod_names: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_information: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_instance: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_comments: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_comment_per_second: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_images: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_image_per_second: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_message: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_message_per_second: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_post: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_post_per_second: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_register: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_register_per_second: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_search: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_search_per_second: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_mode: Option<RegistrationMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_email_verification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sidebar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slur_filter_regex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taglines: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct CustomEmoji {
    pub alt_text: String,
    pub category: String,
    pub id: i32,
    pub image_url: String,
    pub local_site_id: i32,
    pub published: String,
    pub shortcode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct CustomEmojiKeyword {
    pub custom_emoji_id: i32,
    pub id: i32,
    pub keyword: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct CustomEmojiResponse {
    pub custom_emoji_view: CustomEmojiView,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct CustomEmojiView {
    pub custom_emoji: CustomEmoji,
    pub keywords: Vec<CustomEmojiKeyword>,
}

pub struct DeleteAccount {
    pub auth: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct DeleteComment {
    pub auth: String,
    pub comment_id: i32,
    pub deleted: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct DeleteCommunity {
    pub auth: String,
    pub community_id: i32,
    pub deleted: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct DeleteCustomEmoji {
    pub auth: String,
    pub id: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct DeleteCustomEmojiResponse {
    pub id: i32,
    pub success: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct DeletePost {
    pub auth: String,
    pub deleted: bool,
    pub post_id: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct DeletePrivateMessage {
    pub auth: String,
    pub deleted: bool,
    pub private_message_id: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct DistinguishComment {
    pub auth: String,
    pub comment_id: i32,
    pub distinguished: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct EditComment {
    pub auth: String,
    pub comment_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_id: Option<i32>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct EditCommunity {
    pub auth: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,
    pub community_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discussion_languages: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posting_restricted_to_mods: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct EditCustomEmoji {
    pub alt_text: String,
    pub auth: String,
    pub category: String,
    pub id: i32,
    pub image_url: String,
    pub keywords: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct EditPost {
    pub auth: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
    pub post_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct EditPrivateMessage {
    pub auth: String,
    pub content: String,
    pub private_message_id: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct EditSite {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_name_max_length: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_instances: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_email_admins: Option<bool>,
    pub auth: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_instances: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha_difficulty: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub community_creation_admin_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_post_listing_type: Option<ListingType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_theme: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discussion_languages: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_downvotes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_nsfw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub federation_debug: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub federation_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_modlog_mod_names: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_information: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_instance: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_comments: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_comment_per_second: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_images: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_image_per_second: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_message: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_message_per_second: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_post: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_post_per_second: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_register: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_register_per_second: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_search: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_search_per_second: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_mode: Option<RegistrationMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reports_email_admins: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_email_verification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sidebar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slur_filter_regex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taglines: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct FeaturePost {
    pub auth: String,
    pub feature_type: PostFeatureType,
    pub featured: bool,
    pub post_id: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct FederatedInstances {
    pub allowed: Vec<Instance>,
    pub blocked: Vec<Instance>,
    pub linked: Vec<Instance>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct FollowCommunity {
    pub auth: String,
    pub community_id: i32,
    pub follow: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct GetBannedPersons {
    pub auth: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct GetCaptcha {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct GetCaptchaResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    ok: Option<CaptchaResponse>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct GetComment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<String>,
    pub id: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct GetComments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub community_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub community_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_depth: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saved_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<CommentSortType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<ListingType>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct GetCommentsResponse {
    pub comments: Vec<CommentView>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct GetCommunity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct GetCommunityResponse {
    pub community_view: CommunityView,
    pub discussion_languages: Vec<i32>,
    pub moderators: Vec<CommunityModeratorView>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<Site>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct GetFederatedInstances {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct GetFederatedInstancesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub federated_instances: Option<FederatedInstances>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct GetModlog {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub community_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mod_person_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_person_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<ModlogActionType>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct GetModlogResponse {
    pub added: Vec<ModAddView>,
    pub added_to_community: Vec<ModAddCommunityView>,
    pub admin_purged_comments: Vec<AdminPurgeCommentView>,
    pub admin_purged_communities: Vec<AdminPurgeCommunityView>,
    pub admin_purged_persons: Vec<AdminPurgePersonView>,
    pub admin_purged_posts: Vec<AdminPurgePostView>,
    pub banned: Vec<ModBanView>,
    pub banned_from_community: Vec<ModBanFromCommunityView>,
    pub featured_posts: Vec<ModFeaturePostView>,
    pub hidden_communities: Vec<ModHideCommunityView>,
    pub locked_posts: Vec<ModLockPostView>,
    pub removed_comments: Vec<ModRemoveCommentView>,
    pub removed_communities: Vec<ModRemoveCommunityView>,
    pub removed_posts: Vec<ModRemovePostView>,
    pub transferred_to_community: Vec<ModTransferCommunityView>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct GetPersonDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub community_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saved_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<SortType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct GetPersonDetailsResponse {
    pub comments: Vec<CommentView>,
    pub moderates: Vec<CommunityModeratorView>,
    pub person_view: PersonView,
    pub posts: Vec<PostView>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct GetPersonMentions {
    pub auth: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<CommentSortType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unread_only: Option<bool>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct GetPersonMentionsResponse {
    pub mentions: Vec<PersonMentionView>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct GetPost {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct GetPostResponse {
    pub community_view: CommunityView,
    pub cross_posts: Vec<PostView>,
    pub moderators: Vec<CommunityModeratorView>,
    pub post_view: PostView,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct GetPosts {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub community_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub community_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saved_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<SortType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<ListingType>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct GetPostsResponse {
    pub posts: Vec<PostView>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct GetPrivateMEssages {
    pub auth: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unread_only: Option<bool>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct GetReplies {
    pub auth: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<CommentSortType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unread_only: Option<bool>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct GetRepliesResponse {
    pub replies: Vec<CommentReplyView>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct GetReportCount {
    pub auth: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub community_id: Option<i32>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct GetReportCountResponse {
    pub comment_reports: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub community_id: Option<i32>,
    pub post_reports: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_message_reports: Option<i32>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct GetSite {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct GetSiteMetadata {
    pub url: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct GetSiteMetadataResponse {
    pub metadata: SiteMetadata,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct GetSiteResponse {
    pub admins: Vec<PersonView>,
    pub all_languages: Vec<Language>,
    pub custom_emojis: Vec<CustomEmojiView>,
    pub discussion_languages: Vec<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my_user: Option<MyUserInfo>,
    pub site_view: SiteView,
    pub taglines: Vec<Tagline>,
    pub version: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct GetUnreadCount {
    pub auth: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct GetUnreadCountResponse {
    pub mentions: i32,
    pub private_messages: i32,
    pub replies: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct GetUnreadRegistrationApplicationCount {
    pub auth: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct GetUnreadRegistrationApplicationCountResponse {
    pub registration_applications: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct HideCommunity {
    pub auth: String,
    pub community_id: i32,
    pub hidden: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ImageFile {
    pub delete_token: String,
    pub file: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct Instance {
    pub domain: String,
    pub id: i32,
    pub published: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub software: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct Language {
    pub code: String,
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct LeaveAdmin {
    pub auth: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ListCommentReports {
    pub auth: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub community_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unresolved_only: Option<bool>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ListCommentReportsResponse {
    pub comment_reports: Vec<CommentReportView>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ListCommunities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_nsfw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<SortType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<ListingType>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ListCommunitiesResponse {
    pub communities: Vec<CommunityView>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ListPostReports {
    pub auth: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub community_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unresolved_only: Option<bool>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ListPostReportsResponse {
    pub post_reports: Vec<PostReportView>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ListPrivateMessageReports {
    pub auth: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unresolved_only: Option<bool>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ListPrivateMessageReportsResponse {
    pub private_message_reports: Vec<PrivateMessageReportView>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ListRegistrationApplications {
    pub auth: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unresolved_only: Option<bool>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ListRegistrationApplicationsResponse {
    pub registration_applications: Vec<RegistrationApplicationView>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct LocalSite {
    pub actor_name_max_length: i32,
    pub application_email_admins: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_questions: Option<String>,
    pub captcha_difficulty: String,
    pub captcha_enabled: bool,
    pub community_creation_admin_only: bool,
    pub default_post_listing_type: ListingType,
    pub default_theme: String,
    pub enable_downvotes: bool,
    pub enable_nsfw: bool,
    pub federation_enabled: bool,
    pub hide_modlog_mod_names: bool,
    pub id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_information: Option<String>,
    pub private_instance: bool,
    pub published: String,
    pub registration_mode: RegistrationMode,
    pub reports_email_admins: bool,
    pub require_email_verification: bool,
    pub site_id: i32,
    pub site_setup: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slur_filter_regex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct LocalSiteRateLimit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<i32>,
    pub comment_per_second: i32,
    pub id: i32,
    pub image: i32,
    pub image_per_second: i32,
    pub local_site_id: i32,
    pub message: i32,
    pub message_per_second: i32,
    pub post: i32,
    pub post_per_second: i32,
    pub published: String,
    pub register: i32,
    pub register_per_second: i32,
    pub search: i32,
    pub search_per_second: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct LocalUser {
    pub acepted_application: bool,
    pub default_listing_type: ListingType,
    pub default_sort_type: SortType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub email_verified: bool,
    pub id: i32,
    pub interface_language: String,
    pub open_links_in_new_tab: bool,
    pub person_id: i32,
    pub send_notifications_to_email: bool,
    pub show_avatars: bool,
    pub show_bot_accounts: bool,
    pub show_new_post_notifs: bool,
    pub show_nsfw: bool,
    pub show_read_posts: bool,
    pub show_scores: bool,
    pub theme: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totp_2fa_url: Option<String>,
    pub validator_time: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct LocalUserView {
    pub counts: PersonAggregates,
    pub local_user: LocalUser,
    pub person: Person,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct LockPost {
    pub auth: String,
    pub locked: bool,
    pub post_id: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct Login {
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totp_2fa_token: Option<String>,
    pub username_or_email: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct LoginResponse {
    pub jwt: String,
    pub registration_created: bool,
    pub verify_email_sent: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct MarkAllAsRead {
    pub auth: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct MarkCommentReplyAsRead {
    pub auth: String,
    pub comment_reply_id: i32,
    pub read: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct MarkPersonMentionAsRead {
    pub auth: String,
    pub person_mention_id: i32,
    pub read: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct MarkPostAsRead {
    pub auth: String,
    pub post_id: i32,
    pub read: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct MarkPrivateMessageAsRead {
    pub auth: String,
    pub private_message_id: i32,
    pub read: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ModAdd {
    pub id: i32,
    pub mod_person_id: i32,
    pub other_person_id: i32,
    pub removed: bool,
    pub when_: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ModdAddCommunity {
    pub community_id: i32,
    pub id: i32,
    pub mod_person_id: i32,
    pub other_person_id: i32,
    pub removed: bool,
    pub when_: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ModAddCommunityView {
    pub community: Community,
    pub mod_add_community: ModdAddCommunity,
    pub modded_person: Person,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderator: Option<Person>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ModAddView {
    pub mod_add: ModAdd,
    pub modded_person: Person,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderator: Option<Person>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ModBan {
    pub banned: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,
    pub id: i32,
    pub mod_person_id: i32,
    pub other_person_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub when_: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ModBanFromCommunity {
    pub banned: bool,
    pub community_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,
    pub id: i32,
    pub mod_person_id: i32,
    pub other_person_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub when_: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ModBanFromCommunityView {
    pub banned_person: Person,
    pub community: Community,
    pub mod_ban_from_community: ModBanFromCommunity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderator: Option<Person>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ModBanView {
    pub banned_person: Person,
    pub mod_ban: ModBan,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderator: Option<Person>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ModFeaturePost {
    pub featured: bool,
    pub id: i32,
    pub is_featured_community: bool,
    pub mod_person_id: i32,
    pub post_id: i32,
    pub when_: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ModFeaturePostView {
    pub community: Community,
    pub mod_feature_post: ModFeaturePost,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderator: Option<Person>,
    pub post: Post,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ModHideCommunity {
    pub community_id: i32,
    pub hidden: bool,
    pub id: i32,
    pub mod_person_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub when_: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ModHideCommunityView {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin: Option<Person>,
    pub community: Community,
    pub mod_hide_community: ModHideCommunity,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ModJoin {
    pub community_id: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ModJoinResponse {
    pub joined: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ModLockPost {
    pub id: i32,
    pub locked: bool,
    pub mod_person_id: i32,
    pub post_id: i32,
    pub when_: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ModLockPostView {
    pub community: Community,
    pub mod_lock_post: ModLockPost,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderator: Option<Person>,
    pub post: Post,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ModRemoveComment {
    pub comment_id: i32,
    pub id: i32,
    pub mod_person_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub removed: bool,
    pub when_: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ModRemoveCommentView {
    pub comment: Comment,
    pub commenter: Person,
    pub community: Community,
    pub mod_remove_comment: ModRemoveComment,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderator: Option<Person>,
    pub post: Post,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ModRemoveCommunity {
    pub community_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,
    pub id: i32,
    pub mod_person_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub removed: bool,
    pub when_: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ModRemoveCommunityView {
    pub community: Community,
    pub mod_remove_community: ModRemoveCommunity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderator: Option<Person>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ModRemovePost {
    pub id: i32,
    pub mod_person_id: i32,
    pub post_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub removed: bool,
    pub when_: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ModRemovePostView {
    pub community: Community,
    pub mod_remove_post: ModRemovePost,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderator: Option<Person>,
    pub post: Post,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ModTransferCommunity {
    pub community_id: i32,
    pub id: i32,
    pub mod_person_id: i32,
    pub other_person_id: i32,
    pub when_: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ModTransferCommunityView {
    pub community: Community,
    pub mod_transfer_community: ModTransferCommunity,
    pub modded_person: Person,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderator: Option<Person>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ModlogListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub community_id: Option<i32>,
    pub hide_modlog_names: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mod_person_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_person_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct MyUserInfo {
    pub community_blocks: Vec<CommunityBlockView>,
    pub discussion_languages: Vec<i32>,
    pub follows: Vec<CommunityFollowerView>,
    pub local_user_view: LocalUserView,
    pub moderates: Vec<CommunityModeratorView>,
    pub person_blocks: Vec<PersonBlockView>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct PasswordChangeAfterReset {
    pub password: String,
    pub password_verify: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct PasswordReset {
    pub email: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct Person {
    pub actor_id: String,
    pub admin: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ban_expires: Option<String>,
    pub banned: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    pub bot_account: bool,
    pub deleted: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    pub id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_url: Option<String>,
    pub instance_id: i32,
    pub local: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matrix_user_id: Option<String>,
    pub name: String,
    pub published: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct PersonAggregates {
    pub comment_count: i32,
    pub comment_score: i32,
    pub id: i32,
    pub person_id: i32,
    pub post_count: i32,
    pub post_score: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct PersonBlockView {
    pub person: Person,
    pub target: Person,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct PersonMention {
    pub comment_id: i32,
    pub id: i32,
    pub published: String,
    pub read: bool,
    pub receipient_id: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct PersonMentionResponse {
    pub person_mention_view: PersonMentionView,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct PersonMentionView {
    pub comment: Comment,
    pub community: Community,
    pub counts: CommentAggregates,
    pub creator: Person,
    pub creator_banned_from_community: bool,
    pub creator_blocked: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my_vote: Option<i32>,
    pub person_mention: PersonMention,
    pub post: Post,
    pub receipient: Person,
    pub saved: bool,
    pub subscribed: SubscribedType,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct PersonView {
    pub counts: PersonAggregates,
    pub person: Person,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct Post {
    pub ap_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    pub community_id: i32,
    pub creator_id: i32,
    pub deleted: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed_video_url: Option<String>,
    pub featured_community: bool,
    pub featured_local: bool,
    pub id: i32,
    pub language_id: i32,
    pub local: bool,
    pub locked: bool,
    pub name: String,
    pub nsfw: bool,
    pub published: String,
    pub removed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct PostAggregates {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<i32>,
    pub downvotes: i32,
    pub featured_community: bool,
    pub featured_local: bool,
    pub hot_rank: i32,
    pub hot_rank_active: i32,
    pub id: i32,
    pub newest_comment_time: String,
    pub newest_comment_time_necro: String,
    pub upvotes: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct PostJoin {
    pub post_id: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct PostJoinResponse {
    pub joined: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct PostReport {
    pub creator_id: i32,
    pub id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_post_body: Option<String>,
    pub original_post_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_post_url: Option<String>,
    pub post_id: i32,
    pub published: String,
    pub reason: String,
    pub resolved: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct PostReportResponse {
    pub post_report_view: PostReportView,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct PostReportView {
    pub community: Community,
    pub counts: PostAggregates,
    pub creator: Person,
    pub creator_banned_from_community: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my_vote: Option<i32>,
    pub post: Post,
    pub post_creator: Person,
    pub post_report: PostReport,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver: Option<Person>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct PostResponse {
    pub post_view: PostView,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct PostView {
    pub community: Community,
    pub counts: PostAggregates,
    pub creator: Person,
    pub creator_banned_from_community: bool,
    pub creator_blocked: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my_vote: Option<i32>,
    pub post: Post,
    pub read: bool,
    pub saved: bool,
    pub subscribed: SubscribedType,
    pub unread_comments: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct PrivateMessage {
    pub ap_id: String,
    pub content: String,
    pub creator_id: i32,
    pub deleted: bool,
    pub id: i32,
    pub local: bool,
    pub published: String,
    pub read: bool,
    pub recipient_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct PrivateMessageReport {
    pub creator_id: i32,
    pub id: i32,
    pub original_pm_text: String,
    pub private_message_id: i32,
    pub published: String,
    pub reason: String,
    pub resolved: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct PrivateMessageReportResponse {
    pub private_message_report_view: PrivateMessageReportView,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct PrivateMessageReportView {
    pub creator: Person,
    pub private_message: PrivateMessage,
    pub private_message_creator: Person,
    pub private_message_report: PrivateMessageReport,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver: Option<Person>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct PrivateMessageResponse {
    pub private_message_view: PrivateMessageView,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct PrivateMessageView {
    pub creator: Person,
    pub private_message: PrivateMessage,
    pub receipient: Person,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct PrivateMessagesResponse {
    pub private_messages: Vec<PrivateMessageView>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct PurgeComment {
    pub auth: String,
    pub comment_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct PurgeCommunity {
    pub auth: String,
    pub community_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

pub struct PurgeItemResponse {
    pub success: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct PurgePerson {
    pub auth: String,
    pub person_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct PurgePost {
    pub auth: String,
    pub post_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct Register {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha_answer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha_uuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honeypot: Option<String>,
    pub password: String,
    pub password_verify: String,
    pub show_nsfw: bool,
    pub username: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct RegisterationApplication {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_id: Option<i32>,
    pub answer: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deny_reason: Option<String>,
    pub id: i32,
    pub local_user_id: i32,
    pub published: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct RegisterationApplicationResponse {
    pub registeration_application: RegistrationApplicationView,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct RegistrationApplicationView {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin: Option<Person>,
    pub creator: Person,
    pub creator_local_user: LocalUser,
    pub registration_application: RegisterationApplication,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct RemoveComment {
    pub auth: String,
    pub comment_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub removed: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct RemoveCommunity {
    pub auth: String,
    pub community_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub removed: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct RemovePost {
    pub auth: String,
    pub post_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub removed: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ResolveCommentReport {
    pub auth: String,
    pub report_id: i32,
    pub resolved: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ResolveObject {
    pub auth: String,
    pub q: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ResolveObjectResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<CommentView>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub community: Option<CommunityView>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person: Option<PersonView>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<PostView>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ResolvePostReport {
    pub auth: String,
    pub report_id: i32,
    pub resolved: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ResolvePrivateMEssageReport {
    pub auth: String,
    pub report_id: i32,
    pub resolved: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct SaveComment {
    pub auth: String,
    pub comment_id: i32,
    pub save: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct SavePost {
    pub auth: String,
    pub post_id: i32,
    pub save: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct SaveUserSettings {
    pub auth: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_account: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_listing_type: Option<ListingType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_sort_type: Option<SortType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discussion_languages: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_totp_2fa: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interface_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matrix_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_links_in_new_tab: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_notifications_to_email: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_avatars: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_bot_accounts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_new_post_notifs: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_nsfw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_read_posts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_scores: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct Search {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub community_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub community_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listing_type: Option<ListingType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    pub q: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<SortType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<SearchType>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct SearchResponse {
    pub comments: Vec<CommentView>,
    pub communities: Vec<CommunityView>,
    pub posts: Vec<PostView>,
    pub type_: SearchType,
    pub users: Vec<PersonView>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct Site {
    pub actor_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    pub id: i32,
    pub inbox_url: String,
    pub instance_id: i32,
    pub last_refreshed_at: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    pub public_key: String,
    pub published: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sidebar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct SiteAggregates {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<i32>,
    pub communities: i32,
    pub id: i32,
    pub posts: i32,
    pub site_id: i32,
    pub users: i32,
    pub users_active_day: i32,
    pub users_active_half_year: i32,
    pub users_active_month: i32,
    pub users_active_week: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct SiteMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed_video_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct SiteResponse {
    pub site_view: SiteView,
    pub taglines: Vec<Tagline>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct SiteView {
    pub counts: SiteAggregates,
    pub local_site: LocalSite,
    pub local_site_rate_limit: LocalSiteRateLimit,
    pub site: Site,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct Tagline {
    pub content: String,
    pub id: i32,
    pub local_site_id: i32,
    pub published: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct TransferCommunity {
    pub auth: String,
    pub community_id: i32,
    pub person_id: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct UploadImage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<String>,
    // According to HTTP Docs, this needs to be a type of "File"
    pub image: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct UploadImageResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_url: Option<String>,
    pub files: Vec<ImageFile>,
    pub msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct UserJoin {
    pub auth: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct UserJoinResponse {
    pub joined: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct VerifyEmail {
    pub token: String,
}

// The enums for the Lemmy HTTP API, created from the type aliases found here: https://join-lemmy.org/api/index.html
// Also included are some odd "Type Aliases" from the Typescript/HTTP documentation. They hold single values and don't make much sense existing on their own.
// They may eventually be more cleanly integrated into the structs above, for now they will be represetned as structs...

pub struct CommentId {
    pub CommentId: i32,
}

pub struct CommentReplyId {
    pub CommentReplyId: i32,
}

pub struct CommentReportId {
    pub CommentReportId: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub enum CommentSortType {
    #[default]
    Hot,
    Top,
    New,
    Old,
}

impl CommentSortType {
    pub const HOT: &'static str = "Hot";
    pub const TOP: &'static str = "Top";
    pub const NEW: &'static str = "New";
    pub const OLD: &'static str = "Old";

    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            Self::HOT => Some(Self::Hot),
            Self::TOP => Some(Self::Top),
            Self::NEW => Some(Self::New),
            Self::OLD => Some(Self::Old),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Hot => Self::HOT,
            Self::Top => Self::TOP,
            Self::New => Self::NEW,
            Self::Old => Self::OLD,
        }
    }
}

pub struct CommunityBlockId {
    pub CommunityBlockId: i32,
}

pub struct CommunityId {
    pub CommunityId: i32,
}

pub struct CustomEmojiId {
    pub CustomEmojiId: i32,
}

pub struct DeleteAccountResponse {
    pub DeletedAccountResponse: String,
}

pub struct InstanceId {
    pub InstanceId: i32,
}

pub struct LanguageId {
    pub LanguageId: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone, Copy)]
pub enum ListingType {
    All,
    Local,
    #[default]
    Subscribed,
}

impl ListingType {
    pub const ALL: &'static str = "All";
    pub const LOCAL: &'static str = "Local";
    pub const SUBSCRIBED: &'static str = "Subscribed";

    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            Self::ALL => Some(Self::All),
            Self::LOCAL => Some(Self::Local),
            Self::SUBSCRIBED => Some(Self::Subscribed),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::All => Self::ALL,
            Self::Local => Self::LOCAL,
            Self::Subscribed => Self::SUBSCRIBED,
        }
    }
}

pub struct LocalSiteId {
    pub LocalSiteId: i32,
}

pub struct LocalUserId {
    pub LocalUserId: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub enum ModlogActionType {
    #[default]
    All,
    ModRemovePost,
    ModLockPost,
    ModFeaturePost,
    ModRemoveComment,
    ModRemoveCommunity,
    ModBanFromCommunity,
    ModAddCommunity,
    ModTransferCommunity,
    ModAdd,
    ModBan,
    ModHideCommunity,
    AdminPurgePerson,
    AdminPurgeCommunity,
    AdminPurgePost,
    AdminPurgeComment,
}

impl ModlogActionType {
    pub const ALL: &'static str = "All";
    pub const MOD_REMOVE_POST: &'static str = "ModRemovePost";
    pub const MOD_LOCK_POST: &'static str = "ModLockPost";
    pub const MOD_FEATURE_POST: &'static str = "ModFeaturePost";
    pub const MOD_REMOVE_COMMENT: &'static str = "ModRemoveComment";
    pub const MOD_REMOVE_COMMUNITY: &'static str = "ModRemoveCommunity";
    pub const MOD_BAN_FROM_COMMUNITY: &'static str = "ModBanFromCommunity";
    pub const MOD_ADD_COMMUNITY: &'static str = "ModAddCommunity";
    pub const MOD_TRANSFER_COMMUNITY: &'static str = "ModTransferCommunity";
    pub const MOD_ADD: &'static str = "ModAdd";
    pub const MOD_BAN: &'static str = "ModBan";
    pub const MOD_HIDE_COMMUNITY: &'static str = "ModHideCommunity";
    pub const ADMIN_PURGE_PERSON: &'static str = "AdminPurgePerson";
    pub const ADMIN_PURGE_COMMUNITY: &'static str = "AdminPurgeCommunity";
    pub const ADMIN_PURGE_POST: &'static str = "AdminPurgePost";
    pub const ADMIN_PURGE_COMMENT: &'static str = "AdminPurgeComment";

    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            Self::ALL => Some(Self::All),
            Self::MOD_REMOVE_POST => Some(Self::ModRemovePost),
            Self::MOD_LOCK_POST => Some(Self::ModLockPost),
            Self::MOD_FEATURE_POST => Some(Self::ModFeaturePost),
            Self::MOD_REMOVE_COMMENT => Some(Self::ModRemoveComment),
            Self::MOD_REMOVE_COMMUNITY => Some(Self::ModRemoveCommunity),
            Self::MOD_BAN_FROM_COMMUNITY => Some(Self::ModBanFromCommunity),
            Self::MOD_ADD_COMMUNITY => Some(Self::ModAddCommunity),
            Self::MOD_TRANSFER_COMMUNITY => Some(Self::ModTransferCommunity),
            Self::MOD_ADD => Some(Self::ModAdd),
            Self::MOD_BAN => Some(Self::ModBan),
            Self::MOD_HIDE_COMMUNITY => Some(Self::ModHideCommunity),
            Self::ADMIN_PURGE_PERSON => Some(Self::AdminPurgePerson),
            Self::ADMIN_PURGE_COMMUNITY => Some(Self::AdminPurgeCommunity),
            Self::ADMIN_PURGE_POST => Some(Self::AdminPurgePost),
            Self::ADMIN_PURGE_COMMENT => Some(Self::AdminPurgeComment),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::All => Self::ALL,
            Self::ModRemovePost => Self::MOD_REMOVE_POST,
            Self::ModLockPost => Self::MOD_LOCK_POST,
            Self::ModFeaturePost => Self::MOD_FEATURE_POST,
            Self::ModRemoveComment => Self::MOD_REMOVE_COMMENT,
            Self::ModRemoveCommunity => Self::MOD_REMOVE_COMMUNITY,
            Self::ModBanFromCommunity => Self::MOD_BAN_FROM_COMMUNITY,
            Self::ModAddCommunity => Self::MOD_ADD_COMMUNITY,
            Self::ModTransferCommunity => Self::MOD_TRANSFER_COMMUNITY,
            Self::ModAdd => Self::MOD_ADD,
            Self::ModBan => Self::MOD_BAN,
            Self::ModHideCommunity => Self::MOD_HIDE_COMMUNITY,
            Self::AdminPurgePerson => Self::ADMIN_PURGE_PERSON,
            Self::AdminPurgeCommunity => Self::ADMIN_PURGE_COMMUNITY,
            Self::AdminPurgePost => Self::ADMIN_PURGE_POST,
            Self::AdminPurgeComment => Self::ADMIN_PURGE_COMMENT,
        }
    }
}

pub struct PasswordResetResponse {
    pub PasswordResetResponse: String,
}

pub struct PersonBlockId {
    pub PersonBlockId: i32,
}

pub struct PersonId {
    pub PersonId: i32,
}

pub struct PersonMentionId {
    pub PersonMentionId: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub enum PostFeatureType {
    Local,
    #[default]
    Community,
}

impl PostFeatureType {
    pub const LOCAL: &'static str = "Local";
    pub const COMMUNITY: &'static str = "Community";

    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            Self::LOCAL => Some(Self::Local),
            Self::COMMUNITY => Some(Self::Community),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Local => Self::LOCAL,
            Self::Community => Self::COMMUNITY,
        }
    }
}

pub struct PostId {
    pub PostId: i32,
}

pub enum PostOrCommentId {
    Post(PostId),
    Comment(CommentId),
}

pub struct PostReportId {
    pub PostReportId: i32,
}

pub struct PrivateMEssageId {
    pub PrivateMEssageId: i32,
}

pub struct PrivateMessageReportId {
    pub PrivateMessageReportId: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub enum RegistrationMode {
    Closed,
    #[default]
    RequireApplication,
    Open,
}

impl RegistrationMode {
    pub const CLOSED: &'static str = "Closed";
    pub const REQUIRE_APPLICATION: &'static str = "RequireApplication";
    pub const OPEN: &'static str = "Open";

    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            Self::CLOSED => Some(Self::Closed),
            Self::REQUIRE_APPLICATION => Some(Self::RequireApplication),
            Self::OPEN => Some(Self::Open),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Closed => Self::CLOSED,
            Self::RequireApplication => Self::REQUIRE_APPLICATION,
            Self::Open => Self::OPEN,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub enum SearchType {
    #[default]
    All,
    Comments,
    Posts,
    Communities,
    Users,
    Url,
}

impl SearchType {
    pub const ALL: &'static str = "All";
    pub const COMMENTS: &'static str = "Comments";
    pub const POSTS: &'static str = "Posts";
    pub const COMMUNITIES: &'static str = "Communities";
    pub const USERS: &'static str = "Users";
    pub const URL: &'static str = "Url";

    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            Self::ALL => Some(Self::All),
            Self::COMMENTS => Some(Self::Comments),
            Self::POSTS => Some(Self::Posts),
            Self::COMMUNITIES => Some(Self::Communities),
            Self::USERS => Some(Self::Users),
            Self::URL => Some(Self::Url),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::All => Self::ALL,
            Self::Comments => Self::COMMENTS,
            Self::Posts => Self::POSTS,
            Self::Communities => Self::COMMUNITIES,
            Self::Users => Self::USERS,
            Self::Url => Self::URL,
        }
    }
}

pub struct SideId {
    pub SideId: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub enum SortType {
    #[default]
    Active,
    Hot,
    New,
    Old,
    TopDay,
    TopWeek,
    TopMonth,
    TopYear,
    TopAll,
    MostComments,
    NewComments,
    TopHour,
    TopSixHour,
    TopTwelveHour,
    TopThreeMonths,
    TopSixMonths,
    TopNineMonths,
}

impl SortType {
    pub const ACTIVE: &'static str = "Active";
    pub const HOT: &'static str = "Hot";
    pub const NEW: &'static str = "New";
    pub const OLD: &'static str = "Old";
    pub const TOP_DAY: &'static str = "TopDay";
    pub const TOP_WEEK: &'static str = "TopWeek";
    pub const TOP_MONTH: &'static str = "TopMonth";
    pub const TOP_YEAR: &'static str = "TopYear";
    pub const TOP_ALL: &'static str = "TopAll";
    pub const MOST_COMMENTS: &'static str = "MostComments";
    pub const NEW_COMMENTS: &'static str = "NewComments";
    pub const TOP_HOUR: &'static str = "TopHour";
    pub const TOP_SIX_HOUR: &'static str = "TopSixHour";
    pub const TOP_TWELVE_HOUR: &'static str = "TopTwelveHour";
    pub const TOP_THREE_MONTHS: &'static str = "TopThreeMonths";
    pub const TOP_SIX_MONTHS: &'static str = "TopSixMonths";
    pub const TOP_NINE_MONTHS: &'static str = "TopNineMonths";

    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            Self::ACTIVE => Some(Self::Active),
            Self::HOT => Some(Self::Hot),
            Self::NEW => Some(Self::New),
            Self::OLD => Some(Self::Old),
            Self::TOP_DAY => Some(Self::TopDay),
            Self::TOP_WEEK => Some(Self::TopWeek),
            Self::TOP_MONTH => Some(Self::TopMonth),
            Self::TOP_YEAR => Some(Self::TopYear),
            Self::TOP_ALL => Some(Self::TopAll),
            Self::MOST_COMMENTS => Some(Self::MostComments),
            Self::NEW_COMMENTS => Some(Self::NewComments),
            Self::TOP_HOUR => Some(Self::TopHour),
            Self::TOP_SIX_HOUR => Some(Self::TopSixHour),
            Self::TOP_TWELVE_HOUR => Some(Self::TopTwelveHour),
            Self::TOP_THREE_MONTHS => Some(Self::TopThreeMonths),
            Self::TOP_SIX_MONTHS => Some(Self::TopSixMonths),
            Self::TOP_NINE_MONTHS => Some(Self::TopNineMonths),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Active => Self::ACTIVE,
            Self::Hot => Self::HOT,
            Self::New => Self::NEW,
            Self::Old => Self::OLD,
            Self::TopDay => Self::TOP_DAY,
            Self::TopWeek => Self::TOP_WEEK,
            Self::TopMonth => Self::TOP_MONTH,
            Self::TopYear => Self::TOP_YEAR,
            Self::TopAll => Self::TOP_ALL,
            Self::MostComments => Self::MOST_COMMENTS,
            Self::NewComments => Self::NEW_COMMENTS,
            Self::TopHour => Self::TOP_HOUR,
            Self::TopSixHour => Self::TOP_SIX_HOUR,
            Self::TopTwelveHour => Self::TOP_TWELVE_HOUR,
            Self::TopThreeMonths => Self::TOP_THREE_MONTHS,
            Self::TopSixMonths => Self::TOP_SIX_MONTHS,
            Self::TopNineMonths => Self::TOP_NINE_MONTHS,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub enum SubscribedType {
    Subscribed,
    #[default]
    NotSubscribed,
    Pending,
}

impl SubscribedType {
    pub const SUBSCRIBED: &'static str = "Subscribed";
    pub const NOT_SUBSCRIBED: &'static str = "NotSubscribed";
    pub const PENDING: &'static str = "Pending";

    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            Self::SUBSCRIBED => Some(Self::Subscribed),
            Self::NOT_SUBSCRIBED => Some(Self::NotSubscribed),
            Self::PENDING => Some(Self::Pending),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Subscribed => Self::SUBSCRIBED,
            Self::NotSubscribed => Self::NOT_SUBSCRIBED,
            Self::Pending => Self::PENDING,
        }
    }
}

pub struct VerifyEmailResponse {
    pub VerifyEmailResponse: String,
}
