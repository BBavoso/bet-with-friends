# Bet with Friends

Bet with Friends is a web app that allows users to create, join, and manage friendly bets with their friends.
Users can track scores, set custom stakes, and compete in various bet categories.

## Features

-   [ ] User Authentication: Sign up or log in with email or social accounts.
-   [ ] Friend Management: Add, remove, and organize friends or create groups for easy betting.
-   [ ] Create Bets: Users can create custom bets with unique terms and invite friends to participate.
-   [ ] Scoreboard & Leaderboards: Track personal bet history and view global leaderboards based on wins.
-   [ ] Notifications: Get real-time updates on bet invitations, results, and friend activity.
-   [ ] Social Sharing: Share bet results on social media or invite friends to join.
-   [ ] Wagering System: Track stakes using a point or credit system (or integrate real money payment gateways).
-   [ ] Messaging: Chat with friends directly within bets.

# Tech Stack (Subject to Change)

-   Frontend: Svelte
-   Backend: Rust
-   Database: PostgreSQL or another SQL-based system
-   Authentication: To be determined (e.g., OAuth for social logins, JWT)

# Notes on running

-   You must put a postgres URL in `backend/.env`
-   To run the tests, the postgres user needs superuser permissions
