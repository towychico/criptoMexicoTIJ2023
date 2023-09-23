import react from "react"
import './postCard.css'
import CardHeaderQuestion from "./CardHeaderQuestion";
import CardHeaderChallenge from "./CardHeaderChallenge";

interface Props {
    title: string;
    content: string;
    type: number;
}

function PostCard({ title,content,type }: Props){
    return(<div className="postCard">
        {type ? (
            <CardHeaderQuestion/>
        ) : (
            <CardHeaderChallenge/>
        )}
        <h2 className="postCardTitle">{title}</h2>
        <p className="postCardContent">{content}</p>
        <div className="postCardButtonOutsideContainer">
            <div className="postCardButtonInerContainer">
                {/* eslint-disable-next-line jsx-a11y/control-has-associated-label */}
                <button id="postCardLike" className="postCardActionButton" type="button"/>
                {/* eslint-disable-next-line jsx-a11y/control-has-associated-label */}
                <button id="postCardSave" className="postCardActionButton" type="button"/>
                {/* eslint-disable-next-line jsx-a11y/control-has-associated-label */}
                <button id="postCardShare" className="postCardActionButton" type="button"/>
                {/* eslint-disable-next-line jsx-a11y/control-has-associated-label */}
                <button id="postCardBan" className="postCardActionButton" type="button"/>
            </div>
            {/* eslint-disable-next-line jsx-a11y/control-has-associated-label */}
            <button className="cardPostAddButton" type="button"/>
        </div>
    </div>)
}
// eslint-disable-next-line import/no-default-export
export default PostCard;