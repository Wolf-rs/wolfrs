# Project Status

This represents the status of each part of Wolfrs, as well as what needs to be done. There are `TODO` comments at the top of each file that also roughly cover the same things. However, those may not be inclusive, nor entirely up to date as things are added or fixed. Instead, consider this document the core source for information on what is completed, what still needs work, and what needs to be added.

This list is not even close to exhaustive and will be expanded upon in time as need arises.

## Pages

  - ### Communities.rs
    - #### Get Functionality
      - [x] API functionality
      - [x] Basic table implementated
      - [x] Sorting by monthly active users
      - [x] Pagination implemented
      - [] Support for custom sorting options (TOML or in UI)
      - [] Community avatars
      - [] Short description of each community
      - [] Improve styling, especially on mobile
    - ### Post Functionality
      - [] Ability to subscribe to communities from communities list

  - ### Community.rs
    - #### Get Functionality
      - [x] API functionality
      - [x] Basic feed implemented
      - [] Sidebar implemented
      - [] Community information and graphics showing
      - [] Improve styling
    - #### Post Functionality
      - Not started

  - ### Create_Community.rs
    - Not started

  - ### Create_Post.rs
    - Not started

  - ### Federated_Instances.rs
    - #### Get Functionality
      - [x] API Functionality
      - [x] Fully implemented tables for Allowed/Linked and Blocked instances
      - [x] Sorting alphabetically
      - [] Improve styling, especially on mobile

  - ### Home.rs (The main feed)
    - #### Get Functionality
      - [x] API Functionality
      - [x] Feed implementation
      - [x] Designing of individual post items
      - [x] Initial design of functional components (voting, crossposting, etc)
      - [x] Support for viewing by Subscribed (Not implemented yet), Local, and All posts
      - [-] Support for media-posts (image viewing when clicking on thumbnail)
      - [] Implemented sorting (active, hot, top by time, etc)
      - [] Sidebar implemented
      - [] Trending communities box implemented
      - [] Support for server alerts
      - [] Improved styling, especially on mobile
    - ### Post/Put
      - Not started

  - ### Login.rs
    - Not started yet

  - ### Mod.rs
   - No special functionality at this time

  - ### Mod_Log.rs
    - #### Get Functionality
      - [x] API functionality
      - [-] Rudimentary implementation
      - [] Unified log sorted by time 
      - [] Sorting based on types of mod log actions
      - [] Proper table support for mod log
      - [] Improved styling, especially on mobile

  - ### Notifications.rs
    - Not started yet

  - ### Post.rs 
    - #### Get Functionality
      - [x] API Functionality
      - [x] Mostly completed post body implemented
      - [x] Rendering with Markdown (still need to work out superscripts and blockquotes)
      - [x] Support for viewing PostItem (from the feed) at top of post
      - [-] Implement proper media handliong for images, videos, etc
      - [-] Support posts with no body text and only links
      - [] Improve styling, especially on mobile
      - [x] Basic comment support implemented
      - [] Comment replies and sorting implemented
      - [] Improved styling, especially on mobile
    - #### Post/Put
      - Not started yet

  - ### Reports.rs
    - Not started yet

  - ### Search.rs
    - Not started yet

  - ### Settings.rs
    - Not started yet

  - ### User.rs
    - #### Get Functionality
      - [x] API Functionality
      - [x] Basic implementation of feed
      - [] Implemented sorting
      - [] Implemented sidebar for user
      - [] Feed for user comments
      - [] Improved styling
    - #### Post/Put Functionality
      - Not started yet

- ## Components

  - ### Bindings.rs
    - Not currently used, may be deleted

  - ### Comments.rs
    - #### Get Functionality
      - [x] API Functionality
      - [x] Basic implementation for parent comments
      - [-] Fully support for comment designs
      - [] Support for markdown styling
      - [-] Support for comment replies (Discovered this uses the path param)
      - [] Improved styling
    - #### Post/Put
      - Not started yet

  - ### Feed.rs
    - #### Get Functionality
      - [x] API Functionality
      - [x] Basic implementation
      - [x] Support for the main/home feed, community feed, and user feed
      - [x] Support for viewing the Subscribed, Local, and All feeds
      - [] Sorting via active, hot, top by time, etc
      - [] Improve overall styling, especially for mobile
      - [-] Styling of individual feed items
      - [] Improve media handling (Images, video, etc)
      - [] Proper handling of external links via thumbnail previews
    - #### Post/Put Functionality
      - Not started yet

  - ### Footer.rs
    - #### Functionality
      - [x] Fully implemented
      - [x] Styling (May still need some changes for mobile)

  - ### Header.rs
    - #### Functionality
      - [x] Implemented
      - [x] Styling
      - [-] Support for account button (Needs support for authenticated user)

  - ### Instance.rs
    - #### Functionality
      - [x] Implemented
      - [x] Fully working for importing settings from `Instance.toml` file
      - [-] Support for more customization options, such as colour profiles

  - ### Mod.rs
    - No special functionality

  - ### Notifications.rs
    - Not started yet

  - ### Pagination.rs
    - #### Functionality
      - [x] Implemented
      - [x] Styling
      - [x] Independent component for anywhere that needs pages

  - ### Post_View.rs
    - #### Get Functionality
      - [x] Basic implementation
      - [x] Support for PostItem, from the feed, at the top of the post
      - [x] Support for markdown rendering (Still needs support for blockquotes and superscript)
      - [-] Handle posts with no body text (those with external links or media)
      - [] Proper handling of media
      - [-] Improved styling
      - [] Sidebar implemented
      - [-] Comments added to post_view
    - #### Post/Put Functionality
      - Not started yet

  - ### Sidecard.rs
    - Not started yet

  - ### Trending.rs
    - Not started yet

- ## API

  - ### Admin.rs
    - Not sarted yet

  - ### Comment.rs
    - [-] Get (Needs support for comment replies)
    - [] Post
    - [] Put

  - ### Community.rs
    - [x] Get
    - [] Post
    - [] Put

  - ### Federation.rs
    - [x] Get
    - [] Post (for admins)
    - [] Put (for admins)

  - ### Mod.rs
    - [x] API URL Constructor
    - [-] Client-Side API
      - [x] Get
      - [] Post
      - [] Put
    - [-] Server-Side API
      - [x] Get
      - [] Post
      - [] Put

  - ### Posts.rs
    - [x] Get
    - [] Post
    - [] Put

  - ### Private_Message.rs
    - Not started yet

  - ### Search.rs
    - Not started Yet

  - ### Site.rs
   - Not started yet

  - ### Structs.rs
    - [x] Basic implementation
    - [-] Bug fixing
    - [-] Derive functionality settled
    - [] Fix single param types (These are weird...)

  - ### User.rs
    - [-] Get
    - [] Post
    - [] Put

